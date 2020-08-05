import sys
from typing import TYPE_CHECKING

import PySide2
import PySide2.QtWidgets

from readstor import agent, helpers
from readstor.applebooks import applebooks
from readstor.config import config
from readstor.stor import stor

from . import widgets

if TYPE_CHECKING:
    from ..config import GlobalConfig


class MenuBarView(PySide2.QtWidgets.QSystemTrayIcon):
    def __init__(self, config: "GlobalConfig" = config) -> None:
        super().__init__(parent=None)

        self._worker_agent = agent.WorkerAgent(
            max_thread_count=1,
            global_callbacks={
                "started": self._callback__global_ui_disable,
                "complete": self._callback__global_ui_enable,
            },
        )

        self._stor = stor.Stor()
        self._applebooks = applebooks.AppleBooks()

        self._init_dialogs()
        self._init_ui()

    def _init_dialogs(self) -> None:

        self._about_dialog = widgets.AboutDialog()

        self._preferences_dialog = widgets.PreferencesDialog()

        self._applebooks_running_alert_dialog = widgets.AlertDialog(
            message="Apple Books is currently running...",
            details=(
                "Storing now can result in corrupted data. "
                "Please quit Apple Books and run again."
            ),
            action_button={
                "label": "Quit Apple Books",
                "action": self._applebooks.quit,
                "default": True,
            },
        )

    def _init_ui(self) -> None:

        self._menubar_icon = widgets.MenuBarIcon()
        self.setIcon(self._menubar_icon.idle)

        #

        self._menu_item__show_about_dialog = widgets.MenuItem(
            label="About...", actions=[self._about_dialog.show]
        )

        self._menu_item__stor_applebooks = widgets.MenuItem(
            label="Stor Apple Books", actions=[self._action__stor_applebooks],
        )

        self._menu_item__open_stor = widgets.MenuItem(
            label=f"Open {config.app.NAME_PRETTY} in Finder...",
            actions=[self._action__open_stor],
        )

        self._menu_item__open_preferences_dialog = widgets.MenuItem(
            label=f"Open {config.app.NAME_PRETTY} Preferences...",
            actions=[self._preferences_dialog.show],
        )

        self._menu_item__quit = widgets.MenuItem(
            label="Quit", actions=[sys.exit], shortcut="Ctrl+Q",
        )

        #

        menu = PySide2.QtWidgets.QMenu()
        menu.addAction(self._menu_item__show_about_dialog)
        menu.addSeparator()
        menu.addAction(self._menu_item__stor_applebooks)
        menu.addSeparator()
        menu.addAction(self._menu_item__open_stor)
        menu.addAction(self._menu_item__open_preferences_dialog)
        menu.addSeparator()
        menu.addAction(self._menu_item__quit)

        self.setContextMenu(menu)

    def _callback__global_ui_enable(self) -> None:
        self._menu_item__open_preferences_dialog.setEnabled(True)
        self._menu_item__stor_applebooks.setEnabled(True)
        self.setIcon(self._menubar_icon.idle)

    def _callback__global_ui_disable(self) -> None:
        self._menu_item__open_preferences_dialog.setEnabled(False)
        self._menu_item__stor_applebooks.setEnabled(False)
        self.setIcon(self._menubar_icon.busy)

    def _action__open_stor(self) -> None:
        helpers.shell.run(command=["open", config.user.path_stor])

    def _action__stor_applebooks(self) -> None:

        if self._applebooks.is_running():
            self._applebooks_running_alert_dialog.show()
            return

        self._worker_agent.dispatch(
            func=self._stor.stor,
            # TODO: Setup these three callbacks to provide good messaging/feedback.
            # local_callbacks={
            #     "error": on_worker_error,
            #     "result": on_worker_result,
            #     "complete": on_worker_complete,
            # },
        )
