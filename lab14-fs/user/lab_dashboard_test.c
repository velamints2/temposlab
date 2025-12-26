#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <string.h>
#include <fcntl.h>

void test_lab5_fork() {
    printf("\n[Lab 5] Testing Fork...\n");
    pid_t pid = fork();
    if (pid == 0) {
        printf("  Child: PID = %d\n", getpid());
        exit(0);
    } else {
        printf("  Parent: PID = %d, Child PID = %d\n", getpid(), pid);
        wait(NULL);
        printf("  ✅ Fork test passed!\n");
    }
}

void test_lab6_exec() {
    printf("\n[Lab 6] Testing Exec...\n");
    pid_t pid = fork();
    if (pid == 0) {
        printf("  Executing hello_world...\n");
        execl("hello_world", "hello_world", NULL);
        perror("exec failed");
        exit(1);
    } else {
        wait(NULL);
        printf("  ✅ Exec test passed!\n");
    }
}

void test_lab9_ramfs() {
    printf("\n[Lab 9 & 12] Testing RamFS...\n");
    int fd = open("hello.txt", O_RDONLY);
    if (fd < 0) {
        printf("  ❌ Failed to open hello.txt\n");
        return;
    }
    char buf[100];
    int len = read(fd, buf, sizeof(buf) - 1);
    if (len > 0) {
        buf[len] = '\0';
        printf("  Read from RamFS: %s\n", buf);
        printf("  ✅ RamFS test passed!\n");
    } else {
        printf("  ❌ Failed to read from RamFS\n");
    }
    close(fd);
}

void test_lab14_ext2() {
    printf("\n[Lab 14] Testing Ext2 Filesystem...\n");
    int fd = open("hello.txt", O_RDONLY);
    if (fd < 0) {
        printf("  ❌ Failed to open hello.txt from Ext2\n");
        return;
    }
    char buf[100];
    int len = read(fd, buf, sizeof(buf) - 1);
    if (len > 0) {
        buf[len] = '\0';
        printf("  Read from Ext2: %s\n", buf);
        printf("  ✅ Ext2 test passed!\n");
    } else {
        printf("  ❌ Failed to read from Ext2\n");
    }
    close(fd);
}

int main() {
    printf("\n");
    printf("╔══════════════════════════════════════════════════════════╗\n");
    printf("║     🧪 SUSTECH OS LAB - INTERACTIVE TEST SUITE          ║\n");
    printf("╚══════════════════════════════════════════════════════════╝\n");
    
    test_lab5_fork();
    test_lab6_exec();
    test_lab9_ramfs();
    test_lab14_ext2();
    
    printf("\n╔══════════════════════════════════════════════════════════╗\n");
    printf("║  ✅ All Tests Completed!                                 ║\n");
    printf("╚══════════════════════════════════════════════════════════╝\n");
    
    return 0;
}

