#include <stdio.h>
#include <sys/ptrace.h>
#include <unistd.h>


// /usr/include/x86_64-linux-gnu/sys/ptrace.h
// /usr/include/x86_64-linux-gnu/asm/ptrace.h
// /usr/include/linux/ptrace.h
void m()
{
#ifdef PT_DENY_ATTACH
    if (ptrace(PT_DENY_ATTACH, 0, 0, 0) == -1)
    {
        perror("ptrace");
    }
#endif
}

int main()
{
    m();

    // Your existing code here
    printf("Running program with ptrace\n");

    return 0;
}