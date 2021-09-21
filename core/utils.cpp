#include <dlapi.h>

void await(dl::IPromisePtr promise) {
  auto result = promise->wait();
  promise->release();
}
