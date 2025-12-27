package com.jboot.async;

/**
 * Base exception for async operations.
 */
public class AsyncException extends RuntimeException {

    public AsyncException(String message) {
        super(message);
    }

    public AsyncException(String message, Throwable cause) {
        super(message, cause);
    }
}
