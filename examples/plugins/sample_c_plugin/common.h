enum Type
{
  Integer,
  Floating,
  String
};

typedef struct
{
  const char* id;
  const char* desc;
  const enum Type type;
} ActionArg;

typedef struct
{
  const char* id;
  const char* name;
  const char* desc;
  const ActionArg** args;
} Action;

typedef struct
{
  const char* id;
  const char* desc;
  const enum Type type;
} Variable;

typedef struct
{
  const char* id;
  const char* name;
  const char* desc;
  const Variable** variables;
  const Action** actions;

  void* (*fn_init)(void);
  void (*fn_update)(void* state);
  char* (*fn_get_variable)(void* state, char* id);
  void (*fn_run_action)(void* state, char* id);
} Plugin;
