import ctypes

lib = ctypes.cdll.LoadLibrary("./target/release/librust_cache.rlib")


class Cache:
    def __init__(self, capacity):
        lib.Cache_new.argtypes = [ctypes.c_size_t]
        lib.Cache_new.restype = ctypes.c_void_p
        self.obj = lib.Cache_new(capacity)

    def __del__(self):
        lib.Cache_drop(self.obj)

    def get(self, key):
        lib.Cache_get.argtypes = [ctypes.c_void_p, ctypes.c_int32]
        lib.Cache_get.restype = ctypes.c_int32
        return lib.Cache_get(self.obj, key)

    def put(self, key, value):
        lib.Cache_put.argtypes = [ctypes.c_void_p, ctypes.c_int32, ctypes.c_int32]
        lib.Cache_put(self.obj, key, value)


cache = Cache(3)
cache.put(1, 10)
cache.put(2, 20)
cache.put(3, 30)

print(cache.get(1)) # output: 10
print(cache.get(2)) # output: 20
print(cache.get(3)) # output: 30

cache.put(4, 40)
print(cache.get(1)) # output: -1
