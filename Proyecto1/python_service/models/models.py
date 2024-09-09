from pydantic import BaseModel # type: ignore
from typing import List

class LogProcess(BaseModel):
    pid: int
    container_id: str
    name: str
    vsz: int
    rss: int
    memory_usage: float
    cpu_usage: float
    action: str
    timestamp: str


class MemoryStats(BaseModel):
    total_ram: int
    free_ram: int
    used_ram: int

class Data(BaseModel):
    memorystats: MemoryStats
    processes: list
    