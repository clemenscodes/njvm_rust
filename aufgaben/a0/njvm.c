#include <stdio.h>
#include <string.h>

int print_start() {
    printf("Ninja Virtual Machine started\n");
    return 0;
}

int print_stop() {
    printf("Ninja Virtual Machine stopped\n");
    return 0;
}

int print_usage() {
    printf("usage: ./njvm [option] [option] ...\n");
    printf("  --version\t\tshow version and exit\n");
    printf("  --help   \t\tshow this help and exit\n");
    return 0;
}

int print_version() {
    printf("Ninja Virtual Machine version 0 (compiled Sep 23 2015, 10:36:52\n");
    return 0;
}

int print_err(char *arg) {
    printf("unknown command line argument '%s', try './njvm --help'\n", arg);
    return 0;
}

int check_args(char *argv[]) {
    int help_result = strcmp(argv[1], "--help");
    int version_result = strcmp(argv[1], "--version");
    if (help_result == 0) {
        print_usage();
    } else if (version_result == 0) {
        print_version();
    } else {
        print_err(argv[1]);
    }
    return 0;
}

int main(int argc, char *argv[]) {
    if (argc > 1) {
        check_args(argv);
    } else {
        print_start();
        print_stop();
    }
    return 0;
}
