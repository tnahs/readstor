import argparse
import logging
import logging.config
import sys

import PySide2
import PySide2.QtCore
import PySide2.QtWidgets

from .config import config
from .views import views


logger = logging.getLogger(__name__)


def main(args: argparse.Namespace) -> None:

    config.env = args.env

    config.app.setup()
    config.app.init_logger(log_level=args.log_level)

    config.user.setup()
    config.user.load()

    logger.info(f"Running in {config.env} mode.")
    logger.info(f"Application root directory: `{config.app.PATH_HOME}`.")
    logger.info(f"User data directory: `{config.user.path_stor}`.")
    logger.info(f"Database directory: `{config.applebooks.PATH_SOURCE_DATABASES}`.")

    app = PySide2.QtWidgets.QApplication([])
    app.setQuitOnLastWindowClosed(False)
    app.setAttribute(PySide2.QtCore.Qt.AA_UseHighDpiPixmaps, True)

    view = views.MenuBarView(config=config)
    view.show()

    sys.exit(app.exec_())
