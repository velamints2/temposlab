# SUSTECH OS LAB - Comprehensive Test Suite

## 🚀 Quick Start

After the system boots and enters the shell, simply run:

```bash
~ # lab_comprehensive_test
```

This will automatically test all 11 Labs (Lab 3-14) and generate a detailed report.

## 📋 What Gets Tested

### Lab 3 & 4: Logging & System Calls
- ✅ `getpid()` system call
- ✅ `getppid()` system call  
- ✅ `write()` system call

### Lab 5 & 6: Fork & Exec
- ✅ `fork()` creates child process
- ✅ `wait()` reaps child process
- ✅ `exec()` loads new program

### Lab 7: Dynamic RR Scheduler
- ✅ Time slice calculation (pid * 10)
- ✅ Process scheduling behavior

### Lab 8: Semaphore Synchronization
- ✅ P/V operations verified

### Lab 9 & 12: RamFS
- ✅ File creation in RamFS
- ✅ File read/write operations
- ✅ Frame-based storage

### Lab 10: Memory Protection
- ✅ `mprotect()` system call

### Lab 11: Page Fault Handler
- ✅ Instruction page fault handling
- ✅ Load/Store page fault handling
- ✅ Demand paging (lazy allocation)

### Lab 13: VirtIO Block Device
- ✅ Block device detection
- ✅ Read/Write operations

### Lab 14: Ext2 Filesystem
- ✅ Ext2 filesystem mount
- ✅ File read from Ext2 root
- ✅ Content verification

## 📊 Expected Output

The test suite will output:
- Detailed test results for each Lab
- Pass/Fail status for each test case
- Final summary with success rate
- Overall status (All Passed / Some Failed)

## 🛠️ Building

The test program is automatically compiled when you run `make run` or `make build`.

## 💡 Tips

- Run `lab_comprehensive_test` after system boot to verify all functionality
- Check the summary at the end for overall status
- Individual test failures will be marked with ❌

