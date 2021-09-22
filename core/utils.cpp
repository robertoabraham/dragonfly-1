#include <dlapi.h>
#include "result.h"

void await(dl::IPromisePtr promise) {
  promise->wait();
  promise->release();
}

Result<dl::ICameraPtr, int> initialize_camera(dl::IGatewayPtr gateway) {

	gateway->queryUSBCameras();

	auto count = gateway->getUSBCameraCount();
	if (count == 0) {
		return Err(0);
	}

	auto camera = gateway->getUSBCamera(0);
	if (!camera) {
		return Err(0);	
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
