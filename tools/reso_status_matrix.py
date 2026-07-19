"""Canonical RESO StandardStatus -> PHDS ListingStatus matrix.

This is the machine-readable form of the status matrix in
docs/crosswalks/reso-dd20-alignment.md; tests keep the two in sync.
Closed disambiguates through the listing's offering_type. Statuses with
preserve_raw are MLS record-management states that normalize to `other`
and MUST keep the raw StandardStatus value in producer-namespaced extras.
"""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class StatusMapping:
    for_sale: str
    for_lease: str
    preserve_raw: bool = False


STANDARD_STATUS_TO_LISTING_STATUS: dict[str, StatusMapping] = {
    "Coming Soon": StatusMapping("coming_soon", "coming_soon"),
    "Active": StatusMapping("active", "active"),
    "Active Under Contract": StatusMapping("active_under_contract", "active_under_contract"),
    "Pending": StatusMapping("pending", "pending"),
    "Hold": StatusMapping("hold", "hold"),
    "Closed": StatusMapping("sold", "leased"),
    "Canceled": StatusMapping("canceled", "canceled"),
    "Expired": StatusMapping("expired", "expired"),
    "Withdrawn": StatusMapping("withdrawn", "withdrawn"),
    "Incomplete": StatusMapping("other", "other", preserve_raw=True),
    "Delete": StatusMapping("other", "other", preserve_raw=True),
}


def normalize_standard_status(standard_status: str, offering_type: str) -> str:
    """Normalize a RESO StandardStatus for a listing's offering_type.

    Raises KeyError for unknown statuses — importers must route those to
    extras rather than guess.
    """
    mapping = STANDARD_STATUS_TO_LISTING_STATUS[standard_status]
    return mapping.for_lease if offering_type == "for_lease" else mapping.for_sale
