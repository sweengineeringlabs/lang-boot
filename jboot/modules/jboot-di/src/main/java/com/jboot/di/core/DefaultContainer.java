package com.jboot.di.core;

import com.jboot.di.api.Container;
import com.jboot.di.api.ContainerException;

import java.util.Map;
import java.util.Optional;
import java.util.concurrent.ConcurrentHashMap;
import java.util.function.Supplier;

/**
 * Default implementation of the DI container.
 */
public class DefaultContainer implements Container {

    private final Map<RegistrationKey, Registration<?>> registrations = new ConcurrentHashMap<>();
    private final Map<RegistrationKey, Object> singletons = new ConcurrentHashMap<>();
    private final Container parent;

    public DefaultContainer() {
        this.parent = null;
    }

    private DefaultContainer(Container parent) {
        this.parent = parent;
    }

    @Override
    public <T> void registerSingleton(Class<T> type, T instance) {
        registerSingleton(type, null, instance);
    }

    @Override
    public <T> void registerSingleton(Class<T> type, String name, T instance) {
        var key = new RegistrationKey(type, name);
        registrations.put(key, new Registration<>(RegistrationType.SINGLETON, () -> instance));
        singletons.put(key, instance);
    }

    @Override
    public <T> void registerFactory(Class<T> type, Supplier<T> factory) {
        var key = new RegistrationKey(type, null);
        registrations.put(key, new Registration<>(RegistrationType.FACTORY, factory));
    }

    @Override
    public <T> void registerLazySingleton(Class<T> type, Supplier<T> factory) {
        var key = new RegistrationKey(type, null);
        registrations.put(key, new Registration<>(RegistrationType.LAZY_SINGLETON, factory));
    }

    @Override
    @SuppressWarnings("unchecked")
    public <T> T resolve(Class<T> type) {
        return resolve(type, null);
    }

    @Override
    @SuppressWarnings("unchecked")
    public <T> T resolve(Class<T> type, String name) {
        var key = new RegistrationKey(type, name);

        var registration = findRegistration(key);
        if (registration == null) {
            throw new ContainerException("No registration found for " + type.getName() +
                    (name != null ? " with name '" + name + "'" : ""));
        }

        return (T) resolveInternal(key, registration);
    }

    private Registration<?> findRegistration(RegistrationKey key) {
        var registration = registrations.get(key);
        if (registration != null) {
            return registration;
        }
        if (parent instanceof DefaultContainer dc) {
            return dc.findRegistration(key);
        }
        return null;
    }

    private Object resolveInternal(RegistrationKey key, Registration<?> registration) {
        return switch (registration.type()) {
            case SINGLETON -> findSingleton(key);
            case FACTORY -> registration.factory().get();
            case LAZY_SINGLETON -> singletons.computeIfAbsent(key, k -> registration.factory().get());
        };
    }

    private Object findSingleton(RegistrationKey key) {
        var singleton = singletons.get(key);
        if (singleton != null) {
            return singleton;
        }
        if (parent instanceof DefaultContainer dc) {
            return dc.findSingleton(key);
        }
        return null;
    }

    @Override
    @SuppressWarnings("unchecked")
    public <T> Optional<T> tryResolve(Class<T> type) {
        var key = new RegistrationKey(type, null);
        var registration = findRegistration(key);
        if (registration == null) {
            return Optional.empty();
        }
        return Optional.of((T) resolveInternal(key, registration));
    }

    @Override
    public boolean isRegistered(Class<?> type) {
        var key = new RegistrationKey(type, null);
        return findRegistration(key) != null;
    }

    @Override
    public Container createScope() {
        return new DefaultContainer(this);
    }

    private record RegistrationKey(Class<?> type, String name) {
    }

    private enum RegistrationType {
        SINGLETON,
        FACTORY,
        LAZY_SINGLETON
    }

    private record Registration<T>(RegistrationType type, Supplier<T> factory) {
    }
}
