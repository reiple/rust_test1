# 빌드
docker build -t rust/test1:0.0.1 -f docker/Dockerfile .

# 간단한 실행
docker run --rm -it rust/test1:0.0.1