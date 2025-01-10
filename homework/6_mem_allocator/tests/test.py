import unittest
import ctypes
from ctypes import CDLL, c_void_p, c_size_t

# simple malloc setup
def simple_malloc(lib):
    lib.simple_malloc.argtypes = [c_size_t]
    lib.simple_malloc.restype = c_void_p

# wrapped malloc setup
def wrapped_malloc(lib):
    lib.wrapped_malloc.argtypes = [c_size_t]
    lib.wrapped_malloc.restype = c_void_p

class TestMemoryAllocator(unittest.TestCase):
    def setUp(self):
        self.lib = CDLL("../libmalloc.so")

        if not self.lib:
            raise "Could not load libmalloc.so"

        simple_malloc(self.lib)
        wrapped_malloc(self.lib)

    def test_simple_malloc_basic_allocation(self):
        """Test allocation of simple_malloc"""
        ptr = self.lib.simple_malloc(24)
        self.assertIsNotNone(ptr)

        # zero allocation
        ptr = self.lib.simple_malloc(0)
        self.assertIsNone(ptr)

    def test_simple_malloc_alignment(self):
        """Test 8-byte alignment"""
        test_sizes = [1, 3, 7, 8, 9, 15, 16, 17]
        for size in test_sizes:
            ptr = self.lib.simple_malloc(size)
            addr = ctypes.cast(ptr, ctypes.c_void_p).value
            eq = addr % 8
            self.assertEqual(eq, 0, f"Address {addr} not 8-byte aligned for size {size}")

    def test_wrapped_malloc_basic_allocation(self):
        """test wrapped_malloc"""
        ptr = self.lib.wrapped_malloc(24)
        self.assertIsNotNone(ptr)

        # zero allocation
        ptr = self.lib.wrapped_malloc(0)
        self.assertEqual(ptr, None)

    def test_wrapped_malloc_header_integrity(self):
        """header initialization integrity test"""
        size = 24
        ptr = self.lib.wrapped_malloc(size)

        header_ptr = ctypes.cast(ptr, ctypes.c_void_p).value - ctypes.sizeof(ctypes.c_size_t) - ctypes.sizeof(ctypes.c_uint32)
        magic_number = ctypes.cast(header_ptr, ctypes.POINTER(ctypes.c_uint32))[1]

        self.assertEqual(magic_number, 0xDEADBEEF, "Header magic number not properly set")

    def test_multiple_allocations(self):
        """multiple allocations work correctly"""
        ptrs = []
        for i in range(5):
            ptr = self.lib.wrapped_malloc(16)
            self.assertIsNotNone(ptr)
            ptrs.append(ptr)

        # pointers must be different
        ptr_set = set(ptrs)
        self.assertEqual(len(ptr_set), len(ptrs), "Duplicate pointers detected")

if __name__ == '__main__':
    unittest.main()
