package com.jboot.di.api;

import java.util.Optional;
import java.util.function.Supplier;

/**
 * Simple dependency injection container.
 */
public interface Container {

    /**
     * Registers a singleton instance.
     *
     * @param type     The type to register
     * @param instance The singleton instance
     * @param <T>      The type
     */
    <T> void registerSingleton(Class<T> type, T instance);

    /**
     * Registers a factory for creating instances.
     *
     * @param type    The type to register
     * @param factory The factory function
     * @param <T>     The type
     */
    <T> void registerFactory(Class<T> type, Supplier<T> factory);

    /**
     * Registers a singleton factory (lazy initialization).
     *
     * @param type    The type to register
     * @param factory The factory function
     * @param <T>     The type
     */
    <T> void registerLazySingleton(Class<T> type, Supplier<T> factory);

    /**
     * Registers a named singleton instance.
     *
     * @param type     The type to register
     * @param name     The name qualifier
     * @param instance The singleton instance
     * @param <T>      The type
     */
    <T> void registerSingleton(Class<T> type, String name, T instance);

    /**
     * Resolves an instance of the given type.
     *
     * @param type The type to resolve
     * @param <T>  The type
     * @return The resolved instance
     * @throws ContainerException if the type is not registered
     */
    <T> T resolve(Class<T> type);

    /**
     * Resolves a named instance of the given type.
     *
     * @param type The type to resolve
     * @param name The name qualifier
     * @param <T>  The type
     * @return The resolved instance
     * @throws ContainerException if the type is not registered
     */
    <T> T resolve(Class<T> type, String name);

    /**
     * Tries to resolve an instance of the given type.
     *
     * @param type The type to resolve
     * @param <T>  The type
     * @return The resolved instance if registered
     */
    <T> Optional<T> tryResolve(Class<T> type);

    /**
     * Checks if a type is registered.
     *
     * @param type The type to check
     * @return true if registered
     */
    boolean isRegistered(Class<?> type);

    /**
     * Creates a new child container scope.
     *
     * @return A new scoped container
     */
    Container createScope();
}
