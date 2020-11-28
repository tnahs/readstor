import logging
import sys
import threading
import traceback
from types import TracebackType
from typing import Any, Callable, Dict, Optional, Tuple

import PySide2
import PySide2.QtCore


logger = logging.getLogger(__name__)


class WorkerSignals(PySide2.QtCore.QObject):
    """Defines signals for `Worker` objects.

    Note: The signals are decoupled from the `Worker` because `QRunnable` is
    not a subclass of `QObject` and does not support signal emission.

    started -- Emitted at the start of the `Worker.run` method. Emits the
        worker's name via the `threading` module.

    error -- Emitted only if the workers internal `__func` raises an exception
        during execution. Emits a tuple of the exception, traceback and a
        formatted traceback.

    result - Emitted only if the workers internal `__func` executes
        successfully. Emits its return value.

    complete -- Emitted at the end of the `Worker.run` method independent of
        wheather or not any exceptions were raised while excecuting the
        workers internal `__func`. Emits the worker's name via the `threading`
        module."""

    NAMES = [
        "started",
        "error",
        "result",
        "complete",
    ]

    started = PySide2.QtCore.Signal(str)
    error = PySide2.QtCore.Signal(tuple)
    result = PySide2.QtCore.Signal(object)
    complete = PySide2.QtCore.Signal(str)


class Worker(PySide2.QtCore.QRunnable):
    def __init__(self, func: Callable, *args, **kwargs) -> None:
        super(Worker, self).__init__()

        self._signals = WorkerSignals()

        self.__func = func
        self.__args = args
        self.__kwargs = kwargs

    def run(self) -> None:
        """Note: The current QThread can be access by calling the static
        function:

            PySide2.QtCore.QThread.currentThread()
        """

        self.__name = threading.current_thread().name

        self._signals.started.emit(self.__name)

        logger.debug("Thread started.")

        try:
            result = self.__func(*self.__args, **self.__kwargs)
        except Exception:
            logger.exception("Thread encountered an error during execution.")
            exception_obj: Optional[BaseException] = sys.exc_info()[-2]
            traceback_obj: Optional[TracebackType] = sys.exc_info()[-1]
            traceback_formatted: str = traceback.format_exc()
            self._signals.error.emit(
                (exception_obj, traceback_obj, traceback_formatted)
            )
        else:
            self._signals.result.emit(result)
        finally:
            self._signals.complete.emit(self.__name)

        logger.debug("Thread complete.")

    def connect(self, callbacks: Optional[Dict[str, Callable]]) -> None:

        if callbacks is None:
            return

        for key in callbacks.keys():
            if key not in WorkerSignals.NAMES:
                valid_keys = [f"`{name}`" for name in WorkerSignals.NAMES]
                raise AssertionError(
                    f"{self.__class__.__name__}: Invalid key in `callbacks`: "
                    f"{key}. Valid keys are: {', '.join(valid_keys)}."
                )

        for signal_name in WorkerSignals.NAMES:

            callback: Optional[Callable] = callbacks.get(signal_name, None)

            if callback is None:
                continue

            signal: PySide2.QtCore.Signal = getattr(self._signals, signal_name)
            signal.connect(callback)


class WorkerAgent(PySide2.QtCore.QObject):

    __threadpool = PySide2.QtCore.QThreadPool()

    def __init__(
        self,
        max_thread_count: Optional[int] = None,
        global_callbacks: Optional[Dict[str, Callable]] = None,
    ) -> None:
        super(WorkerAgent, self).__init__(parent=None)
        """ Manages instantiation and dispatch of worker to threads in a
        QThreadPool. Each worker is connected to a set of global and local
        callbacks to respective Qt signals.

        max_thread_count -- Sets the maximum number of concurrent threads
            allowed in the threadpool. By default this is set to the number of
            processor cores, both real and logical, in the system.

        global_callbacks -- Global callbacks are connected to every worker
            created by the `WorkerAgent`. Each callback is connected to one of
            four worker signals (found in `WorkerSignals`) through a dictionary
            of key:value pairs where the key is signal name and the value is
            the callable. Valid keys are: `started`, `error`, `result`
            and `complete` (found in `WorkerSignals.NAMES`):

                callbacks = {
                    "started": on_worker_started,
                    "error": on_worker_error,
                    "result": on_worker_result,
                    "complete": on_worker_complete,
                }

        https://doc.qt.io/qtforpython/PySide2/QtCore/QThread.html
        https://doc.qt.io/qtforpython/PySide2/QtCore/QThreadPool.html """

        self._global_callbacks = global_callbacks

        if max_thread_count is not None:
            self.max_thread_count = max_thread_count

    def dispatch(
        self,
        func: Callable,
        args: Optional[Tuple[Any, ...]] = None,
        kwargs: Optional[Dict[str, Any]] = None,
        local_callbacks: Optional[Dict[str, Callable]] = None,
    ) -> None:
        """Creates a `Worker` from a callable object and dispatches it to a
        `QThreadPool`.

        func -- The function to call in the thread.
        args -- The function arguments.
        kwargs -- The function keyword arguments.
        local_callbacks -- Local callback are identical to global ones but are
            connected only the current worker. See `global_callbacks`."""

        if args is None:
            args = ()

        if kwargs is None:
            kwargs = {}

        worker = Worker(func, *args, **kwargs)
        worker.connect(callbacks=self._global_callbacks)
        worker.connect(callbacks=local_callbacks)

        logger.debug("Dispatching thread.")

        self.__threadpool.start(worker)

    @property
    def active_threads(self) -> int:
        return self.__threadpool.activeThreadCount()

    @property
    def max_thread_count(self) -> int:
        return self.__threadpool.maxThreadCount()

    @max_thread_count.setter
    def max_thread_count(self, value: int) -> None:
        self.__threadpool.setMaxThreadCount(value)
