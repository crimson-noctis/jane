#include <stdio.h>
#include <string.h>
#include "jane.h"

int main(void) {
    Repl *repl = repl_new();
    repl_run_examples(repl);

    char line[1024];
    bool quit = repl_should_quit(repl);
    while (!quit) {
        printf("> ");
        fflush(stdout);

        if (!fgets(line, sizeof(line), stdin)) break;

        // Strip newline
        line[strcspn(line, "\n")] = '\0';

        char *result = repl_command(repl, line);
        if (result && *result) printf("%s\n", result);
        repl_free_string(result);
        quit = repl_should_quit(repl);
    }

    repl_free(repl);
    return 0;
}
