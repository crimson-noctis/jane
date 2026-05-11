#ifndef JANE_H
#define JANE_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Repl Repl;

struct Repl *repl_new(void);

void repl_run_examples(struct Repl *repl);

char *repl_command(struct Repl *repl, const char *cmd);

bool repl_should_quit(struct Repl *repl);

void repl_free_string(char *s);

void repl_free(struct Repl *repl);

#endif  /* JANE_H */
