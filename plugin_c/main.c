#include <stdlib.h>

#define export __declspec(dllexport)

export struct Plugin {
  char* (*get_name)(struct Plugin*);
  char* (*get_description)(struct Plugin*);
  char* (*get_id)(struct Plugin*);
  char* (*get_actions)(struct Plugin*);
  char* (*get_variables)(struct Plugin*);
  void (*execute_action)(struct Plugin*, char* id);
  void (*update)(struct Plugin*);
};

char* get_name(struct Plugin* self) { return "C Plugin"; }

char* get_description(struct Plugin* self) {}

char* get_id(struct Plugin* self) {}

char* get_actions(struct Plugin* self) {}

char* get_variables(struct Plugin* self) {}

void execute_action(struct Plugin* self, char* id) {}

void update(struct Plugin* self) {}

export void* make() {
  struct Plugin* plugin = malloc(sizeof(struct Plugin));

  plugin->get_name = &get_name;
  plugin->get_description = &get_description;
  plugin->get_id = &get_id;
  plugin->get_actions = &get_actions;
  plugin->get_variables = &get_variables;
  plugin->execute_action = &execute_action;
  plugin->update = &update;

  return plugin;
}