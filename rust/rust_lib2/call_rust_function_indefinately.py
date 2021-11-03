import ctypes
from pydantic import BaseModel
from typing import List
import random

rust = ctypes.CDLL("target/release/librust_lib2.so")


class PythonModel(BaseModel):
    timeout: int
    retries: int
    host_list: List[str]
    action: str


class RustResult(BaseModel):
    result: str
    message: str
    failed_hosts: List[str]


if __name__ == "__main__":

    i = 10000

    while i > 0:
        random_number = random.randint(100, 1000)
        hosts = [f"server-{x}" for x in range(1, random_number)]
        model = PythonModel(
            timeout=10,
            retries=3,
            action="reboot",
            host_list=hosts,
        )
        some_bytes = model.json().encode("utf-8")

        ptr = rust.start_procedure(some_bytes)
        print(ptr)

        returned_bytes = ctypes.c_char_p(ptr).value

        if returned_bytes:
            returned_model = RustResult.parse_raw(returned_bytes)
        hosts = None

        # rust.free_string(ptr)
