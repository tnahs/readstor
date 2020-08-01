import logging
import re
from datetime import datetime
from typing import Optional


logger = logging.getLogger(__name__)


class DateTimeUtilsMixin:
    @staticmethod
    def datetime_from_epoch(epoch: Optional[float], cocoa: bool = True) -> datetime:

        if epoch is None:
            return datetime(1, 1, 1)

        try:
            epoch = float(epoch)
        except TypeError:
            logger.warning(f"Invalid `epoch` date `{epoch}`.")
            return datetime(1, 1, 1)

        if cocoa is True:

            """ Core Data timestamp is the number of seconds (or nanoseconds) since
            midnight, January 1, 2001, GMT (see CFAbsoluteTime). The difference between
            a Core Data timestamp and a Unix timestamp (seconds since 1/1/1970) is
            978307200 seconds.

            https://www.epochconverter.com/coredata """

            epoch = epoch + 978307200.0

        try:
            return datetime.utcfromtimestamp(epoch)
        except ValueError:
            """ `ValueError` typically refers to `ValueError: year 0 is out of
            range` The `datetime` library does not have a year 0. If this is
            raised when called by an `AppleBooksSource` to parse its
            `last_opened_date` it signifies the source was never opened. """
            return datetime(1, 1, 1)

    @staticmethod
    def datetime_from_iso(iso: Optional[str]) -> datetime:

        if iso is None:
            return datetime(1, 1, 1)

        try:
            return datetime.fromisoformat(iso)
        except ValueError:
            logger.warning(f"Invalid `iso` date `{iso}`.")
            return datetime(1, 1, 1)


class EPUBUtilsMixin:
    @staticmethod
    def location_from_epubcfi(epubcfi: Optional[str]) -> str:
        """ https://github.com/matttrent/ibooks-highlights/blob/master/ibooks_highlights/util.py#L20
        """

        if epubcfi is None:
            return ""

        # Starting with: epubcfi(/6/20[part01]!/4/182,/1:0,/3:23)

        # Captures one or more numbers after a slash /0 /00 /000 etc.
        re_digits = re.compile(pattern=r"\/(\d+)")

        # "/6/20[part01]!/4/182,/1:0,/3:23"
        core = epubcfi[8:-1]
        # ["/6/20[part01]!/4/182", "/1:0", "/3:23"]
        core_parts = core.split(",")
        # "/6/20[part01]!/4/182/1:0"
        head = core_parts[0] + core_parts[1]
        # ["/6/20[part01]!/4/182/1", "0"]
        head_parts = head.split(":")
        # [6, 20, 4, 182, 1]
        offsets = [int(x) for x in re.findall(re_digits, head_parts[0])]

        try:
            # [6, 20, 4, 182, 1, 0]
            offsets.append(int(head_parts[1]))
        except IndexError:
            pass

        # 0006.0020.0004.0182.0001.0000
        return ".".join([f"{i:04}" for i in offsets])
