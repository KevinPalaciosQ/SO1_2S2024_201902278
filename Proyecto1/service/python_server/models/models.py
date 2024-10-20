from pydantic import BaseModel # type: ignore
from typing import List

class LogProcess(BaseModel):
    pid: int
    name: str
    cmdline: str
    vsz: int
    rss: int
    memory_usage: float
    cpu_usage: float

class MemoryStats(BaseModel):
    total_ram: int
    free_ram: int
    used_ram: int

class Data(BaseModel):
    timestamp: str
    memorystats: MemoryStats
    processes: List[LogProcess]