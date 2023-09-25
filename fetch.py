import re
import urllib
import requests
from typing import (
    TYPE_CHECKING,
    Callable,
    Dict,
    Any,
    Iterable,
    Iterator,
    List,
    MutableMapping,
    NamedTuple,
    Optional,
    Sequence,
    Tuple,
    Union,
)
import itertools
from itertools import filterfalse, tee, zip_longest
from html.parser import HTMLParser

class HTMLLinkParser(HTMLParser):

    def __init__(self, url: str) -> None:
        super().__init__(convert_charrefs=True)

        self.url: str = url
        self.base_url: Optional[str] = None
        self.anchors: List[Dict[str, Optional[str]]] = []

    def handle_starttag(self, tag: str, attrs: List[Tuple[str, Optional[str]]]) -> None:
        if tag == "base" and self.base_url is None:
            href = self.get_href(attrs)
            if href is not None:
                self.base_url = href
        elif tag == "a":
            self.anchors.append(dict(attrs))

    def get_href(self, attrs: List[Tuple[str, Optional[str]]]) -> Optional[str]:
        for name, value in attrs:
            if name == "href":
                return value
        return None


import re


_SUPPORTED_HASHES = ("sha512", "sha384", "sha256", "sha224", "sha1", "md5")

_hash_url_fragment_re = re.compile(r"[#&]({choices})=([^&]*)".format(
            choices="|".join(re.escape(hash_name) for hash_name in _SUPPORTED_HASHES)
        ))


def fetch():
    session = requests.Session()

    url = "https://pypi.org/simple/jupyterlab/"
    # url = "https://pypi.tuna.tsinghua.edu.cn/simple/jupyterlab/"

    resp = session.get(
        url,
        headers={
            "Accept": ", ".join(
                [
                    # "application/vnd.pypi.simple.v1+json",
                    "application/vnd.pypi.simple.v1+html; q=0.1",
                    "text/html; q=0.01",
                ]
            ),
            "Cache-Control": "max-age=0",
        },
    )    


    parser = HTMLLinkParser(resp.url)
    parser.feed(resp.text)

    # url = page.url
    base_url = parser.base_url or url
    for anchor in parser.anchors:
        # print(anchor)
        href = anchor.get("href")
        if not href:
            return None

        url = _ensure_quoted_url(urllib.parse.urljoin(base_url, href))
        pyrequire = anchor.get("data-requires-python")

        print(pyrequire, url)

        match = _hash_url_fragment_re.search(url)
        # print(333, match)
        if match is not None:
            print(333, match.group(1), match.group(2))

        # link = Link.from_element(anchor, page_url=url, base_url=base_url)
        # if link is None:
            # continue
        # yield link

    # print(resp.text)

def _ensure_quoted_url(url: str) -> str:
    """
    Make sure a link is fully quoted.
    For example, if ' ' occurs in the URL, it will be replaced with "%20",
    and without double-quoting other characters.
    """
    # Split the URL into parts according to the general structure
    # `scheme://netloc/path;parameters?query#fragment`.
    result = urllib.parse.urlparse(url)
    print(222, result)
    is_local_path = not result.netloc
    path = _clean_url_path(result.path, is_local_path=is_local_path)
    print("path: ", path)
    print("aa: ", result._replace(path=path))
    print()
    return urllib.parse.urlunparse(result._replace(path=path))

_reserved_chars_re = re.compile("(@|%2F)", re.IGNORECASE)

def _clean_url_path(path: str, is_local_path: bool) -> str:
    """
    Clean the path portion of a URL.
    """
    if is_local_path:
        clean_func = _clean_file_url_path
    else:
        clean_func = _clean_url_path_part

    # Split on the reserved characters prior to cleaning so that
    # revision strings in VCS URLs are properly preserved.
    parts = _reserved_chars_re.split(path)
    print(111, parts)

    cleaned_parts = []
    for to_clean, reserved in pairwise(itertools.chain(parts, [""])):
        cleaned_parts.append(clean_func(to_clean))
        # Normalize %xx escapes (e.g. %2f -> %2F)
        cleaned_parts.append(reserved.upper())

    return "".join(cleaned_parts)

def _clean_url_path_part(part: str) -> str:
    """
    Clean a "part" of a URL path (i.e. after splitting on "@" characters).
    """
    # We unquote prior to quoting to make sure nothing is double quoted.
    return urllib.parse.quote(urllib.parse.unquote(part))

def pairwise(iterable: Iterable[Any]) -> Iterator[Tuple[Any, Any]]:
    """
    Return paired elements.

    For example:
        s -> (s0, s1), (s2, s3), (s4, s5), ...
    """
    iterable = iter(iterable)
    return zip_longest(iterable, iterable)

# hash = hash_file(local_file.path)[0].hexdigest()
def hash_file(path: str, blocksize: int = 1 << 20) -> Tuple[Any, int]:
    """Return (hash, length) for path using hashlib.sha256()"""

    h = hashlib.sha256()
    length = 0
    with open(path, "rb") as f:
        for block in read_chunks(f, size=blocksize):
            length += len(block)
            h.update(block)
    return h, length


# fetch()

from pip._internal.network.session import user_agent

print(_hash_url_fragment_re)
print(user_agent())
