#include <dlapi.h>
#include "result.h"

void await(dl::IPromisePtr promise) {
  promise->wait();
  promise->release();
}

Result<dl::ICameraPtr, const char *> initialize_camera(dl::IGatewayPtr gateway) {

  gateway->queryUSBCameras();

	auto count = gateway->getUSBCameraCount();
	if (count == 0) {
		return Err("No cameras found!");
	}

	auto camera = gateway->getUSBCamera(0);
	if (!camera) {
		return Err("Could not get camera!");	
	}

	camera->initialize();

  return Ok(camera);
}

dl::IGatewayPtr initialize_gateway() {
  return dl::getGateway();
}

void free_gateway(dl::IGatewayPtr gateway) {
  dl::deleteGateway(gateway);
}

Result<dl::ISensorPtr, const char *> initialize_sensor(dl::ICameraPtr camera) {
  auto sensor = camera->getSensor(0);
  if (!sensor) {
    return Err("Could not initialize sensor!");
  }
  return Ok(sensor);
}

Result<dl::ITECPtr, const char *> initialize_cooler(dl::ICameraPtr camera) {
  auto cooler = camera->getTEC();
  if (!cooler) {
    return Err("Could not initialize cooler!");
  }
  return Ok(cooler);
}
