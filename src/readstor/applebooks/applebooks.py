import logging

from readstor import helpers
from readstor.config import config


logger = logging.getLogger(__name__)


class AppleBooks:
    def is_running(self) -> bool:
        """ Checks to see if Apple Books is currently running. """
        return helpers.shell.process_is_running(process_names=config.applebooks.NAMES)

    def quit(self) -> None:
        """ Kindly asks Apple Books to quit. """
        helpers.shell.run(
            ["osascript", "-e", f'tell application "{config.applebooks.NAME}" to quit',]
        )
