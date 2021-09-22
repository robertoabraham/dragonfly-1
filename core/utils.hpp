#pragma once
#include <dlapi.h>
#include "result.h"

struct SensorInfo {
  unsigned int pixels_x;
  unsigned int pixels_y;
  float pixel_size_x;
  float pixel_size_y;
  float cooler_setpoint_min;
  float cooler_setpoint_max;
  unsigned int bin_x_max;
  unsigned int bin_y_max;
  float exposure_duration_min;
  float exposure_precision;
};

struct CoolerInfo {
  bool cooler_enabled;
  float cooler_power;
  float cooler_setpoint;
  float heatsink_temp;
  float sensor_temp;
};

enum ReadoutMode {
  Low = 0,
  Medium = 1,
  High = 2,
  LowStackPro = 3,
  MediumStackPro = 4,
  HighStackPro = 5,
};

struct ExposureInfo {
  float duration;
  bool light;
  enum ReadoutMode readout_mode;
  int bin_x;
  int bin_y;
};

struct ExposeResult {
  unsigned short *buffer;
  size_t buffer_size;
};


void await(dl::IPromisePtr promise);

dl::IGatewayPtr initialize_gateway();
void free_gateway(dl::IGatewayPtr gateway);

Result<dl::ICameraPtr, const char *> initialize_camera(dl::IGatewayPtr gateway);
Result<dl::ISensorPtr, const char *> initialize_sensor(dl::ICameraPtr camera);
Result<dl::ITECPtr, const char *> initialize_cooler(dl::ICameraPtr camera);

void print_fits_err(int status);
