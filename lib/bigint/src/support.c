#include <stdio.h>
#include <stdlib.h>

#include "support.h"

void fatalError(char *msg) {
  fprintf(stderr, "Error: %s\n", msg);
  exit(1);
}

ObjRef newPrimObject(int dataSize) {
  ObjRef obj_ref = malloc(dataSize + sizeof(int));
  if (!obj_ref) {
    fatalError("Error: failed to allocate memory for obj_ref");
  }
  obj_ref->size = dataSize;
  return obj_ref;
}
