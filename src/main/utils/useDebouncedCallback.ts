import { useEffect, useRef } from 'react';

export function useDebouncedCallback<A extends any[]>(callback: (...args: A) => void, ms: number) {
  const argsRef = useRef<A>();
  const timeout = useRef<ReturnType<typeof setTimeout>>();

  function cleanup() {
    if (timeout.current) {
      clearTimeout(timeout.current);
    }
  }

  useEffect(() => cleanup, []);

  return function debouncedCallback(...args: A) {
    argsRef.current = args;

    cleanup();

    timeout.current = setTimeout(() => {
      if (argsRef.current) {
        callback(...argsRef.current);
      }
    }, ms);
  };
}
