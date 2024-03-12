USERNAME = tempestmon
PROJECT = sandbox
APP_VERSION = v0.0.1
build_release:
	docker build -t ${USERNAME}/${PROJECT}:${APP_VERSION} -f Dockerfile .