package com.jboot.di.core;

import com.jboot.di.api.Container;
import com.jboot.di.api.ContainerException;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Nested;

import java.util.concurrent.atomic.AtomicInteger;

import static org.assertj.core.api.Assertions.*;

class DefaultContainerTest {

    private Container container;

    @BeforeEach
    void setUp() {
        container = new DefaultContainer();
    }

    @Nested
    @DisplayName("Singleton registration")
    class SingletonTests {
        @Test
        void registerAndResolveSingleton() {
            var service = new TestService("singleton");
            container.registerSingleton(TestService.class, service);

            var resolved = container.resolve(TestService.class);

            assertThat(resolved).isSameAs(service);
        }

        @Test
        void singletonReturnsSameInstance() {
            var service = new TestService("singleton");
            container.registerSingleton(TestService.class, service);

            var first = container.resolve(TestService.class);
            var second = container.resolve(TestService.class);

            assertThat(first).isSameAs(second);
        }

        @Test
        void namedSingletonReturnsCorrectInstance() {
            var prod = new TestService("prod");
            var test = new TestService("test");

            container.registerSingleton(TestService.class, "prod", prod);
            container.registerSingleton(TestService.class, "test", test);

            assertThat(container.resolve(TestService.class, "prod")).isSameAs(prod);
            assertThat(container.resolve(TestService.class, "test")).isSameAs(test);
        }
    }

    @Nested
    @DisplayName("Factory registration")
    class FactoryTests {
        @Test
        void factoryCreatesNewInstance() {
            var counter = new AtomicInteger(0);
            container.registerFactory(TestService.class,
                    () -> new TestService("instance-" + counter.incrementAndGet()));

            var first = container.resolve(TestService.class);
            var second = container.resolve(TestService.class);

            assertThat(first).isNotSameAs(second);
            assertThat(first.name()).isEqualTo("instance-1");
            assertThat(second.name()).isEqualTo("instance-2");
        }

        @Test
        void factoryIsCalledEachTime() {
            var counter = new AtomicInteger(0);
            container.registerFactory(TestService.class, () -> {
                counter.incrementAndGet();
                return new TestService("test");
            });

            container.resolve(TestService.class);
            container.resolve(TestService.class);
            container.resolve(TestService.class);

            assertThat(counter.get()).isEqualTo(3);
        }
    }

    @Nested
    @DisplayName("Lazy singleton registration")
    class LazySingletonTests {
        @Test
        void lazySingletonCreatesInstanceOnFirstResolve() {
            var counter = new AtomicInteger(0);
            container.registerLazySingleton(TestService.class, () -> {
                counter.incrementAndGet();
                return new TestService("lazy");
            });

            assertThat(counter.get()).isEqualTo(0); // Not created yet

            container.resolve(TestService.class);
            assertThat(counter.get()).isEqualTo(1);

            container.resolve(TestService.class);
            assertThat(counter.get()).isEqualTo(1); // Same instance
        }

        @Test
        void lazySingletonReturnsSameInstance() {
            container.registerLazySingleton(TestService.class,
                    () -> new TestService("lazy"));

            var first = container.resolve(TestService.class);
            var second = container.resolve(TestService.class);

            assertThat(first).isSameAs(second);
        }
    }

    @Nested
    @DisplayName("Resolution failures")
    class ResolutionFailureTests {
        @Test
        void throwsForUnregisteredType() {
            assertThatThrownBy(() -> container.resolve(TestService.class))
                    .isInstanceOf(ContainerException.class)
                    .hasMessageContaining("No registration found");
        }

        @Test
        void throwsForUnregisteredNamedType() {
            container.registerSingleton(TestService.class, new TestService("default"));

            assertThatThrownBy(() -> container.resolve(TestService.class, "nonexistent"))
                    .isInstanceOf(ContainerException.class)
                    .hasMessageContaining("nonexistent");
        }
    }

    @Nested
    @DisplayName("Try resolve")
    class TryResolveTests {
        @Test
        void returnsEmptyForUnregistered() {
            var result = container.tryResolve(TestService.class);

            assertThat(result).isEmpty();
        }

        @Test
        void returnsPresentForRegistered() {
            container.registerSingleton(TestService.class, new TestService("test"));

            var result = container.tryResolve(TestService.class);

            assertThat(result).isPresent();
        }
    }

    @Nested
    @DisplayName("Registration check")
    class IsRegisteredTests {
        @Test
        void returnsFalseForUnregistered() {
            assertThat(container.isRegistered(TestService.class)).isFalse();
        }

        @Test
        void returnsTrueForRegistered() {
            container.registerSingleton(TestService.class, new TestService("test"));

            assertThat(container.isRegistered(TestService.class)).isTrue();
        }
    }

    @Nested
    @DisplayName("Scoped containers")
    class ScopeTests {
        @Test
        void scopeInheritsParentRegistrations() {
            container.registerSingleton(TestService.class, new TestService("parent"));

            var scope = container.createScope();

            var resolved = scope.resolve(TestService.class);
            assertThat(resolved.name()).isEqualTo("parent");
        }

        @Test
        void scopeCanOverrideParentRegistrations() {
            container.registerSingleton(TestService.class, new TestService("parent"));

            var scope = container.createScope();
            scope.registerSingleton(TestService.class, new TestService("child"));

            assertThat(container.resolve(TestService.class).name()).isEqualTo("parent");
            assertThat(scope.resolve(TestService.class).name()).isEqualTo("child");
        }

        @Test
        void nestedScopesWorkCorrectly() {
            container.registerSingleton(String.class, "root");

            var level1 = container.createScope();
            level1.registerSingleton(Integer.class, 1);

            var level2 = level1.createScope();
            level2.registerSingleton(Double.class, 2.0);

            // Level 2 can resolve all
            assertThat(level2.resolve(String.class)).isEqualTo("root");
            assertThat(level2.resolve(Integer.class)).isEqualTo(1);
            assertThat(level2.resolve(Double.class)).isEqualTo(2.0);

            // Level 1 can resolve root and its own
            assertThat(level1.resolve(String.class)).isEqualTo("root");
            assertThat(level1.resolve(Integer.class)).isEqualTo(1);
            assertThat(level1.isRegistered(Double.class)).isFalse();
        }
    }

    @Nested
    @DisplayName("Interface registration")
    class InterfaceTests {
        @Test
        void canRegisterImplementation() {
            container.registerSingleton(Greeter.class, new EnglishGreeter());

            var greeter = container.resolve(Greeter.class);

            assertThat(greeter.greet("World")).isEqualTo("Hello, World!");
        }

        @Test
        void multipleImplementationsWithNames() {
            container.registerSingleton(Greeter.class, "english", new EnglishGreeter());
            container.registerSingleton(Greeter.class, "french", new FrenchGreeter());

            assertThat(container.resolve(Greeter.class, "english").greet("World"))
                    .isEqualTo("Hello, World!");
            assertThat(container.resolve(Greeter.class, "french").greet("World"))
                    .isEqualTo("Bonjour, World!");
        }
    }

    // Test helpers
    record TestService(String name) {
    }

    interface Greeter {
        String greet(String name);
    }

    static class EnglishGreeter implements Greeter {
        @Override
        public String greet(String name) {
            return "Hello, " + name + "!";
        }
    }

    static class FrenchGreeter implements Greeter {
        @Override
        public String greet(String name) {
            return "Bonjour, " + name + "!";
        }
    }
}
