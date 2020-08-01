from typing import Optional


class AppleBooksUtilsMixin:
    @staticmethod
    def style_from_index(index: Optional[int]) -> str:
        """ Converts AppleBooks style index to style string. """

        if index is None:
            return ""

        return {
            0: "underline",
            1: "green",
            2: "blue",
            3: "yellow",
            4: "pink",
            5: "purple",
        }.get(index, "")
