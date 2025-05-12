#define export __declspec(dllexport)

const char *__NAME = "CLang Plugin";
const char *__DESCRIPTION = "CLang Plugin";
const char *__ID = "CLang Plugin";

export const char *get_name() { return __NAME; }

export const char *get_description() { return __DESCRIPTION; }
export const char *get_id() { return __ID; }

struct Plugin {
  void *(*new)(void);

  void (*update)(void *state);
  void (*execute_action)(void *state, char *id);
};

void *new() { return 0; }

void update(void *state) { return; }

void execute_action(void *state, char *id) { return; }

export const struct Plugin PLUGIN = {new, update, execute_action};
