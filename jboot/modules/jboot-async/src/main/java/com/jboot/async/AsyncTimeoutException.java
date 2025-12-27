package com.jboot.async;

/**
 * Exception thrown when an async operation times out.
 */
public class AsyncTimeoutException extends AsyncException {

    public AsyncTimeoutException(String message) {
        super(message);
    }

    public AsyncTimeoutException(String message, Throwable cause) {
        super(message, cause);
    }
}
