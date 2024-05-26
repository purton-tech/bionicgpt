## Expose Selenium Ports

Setup all the ENV vars 

```sh
get-env
```

Expose the ports in k9s

## Run selenium

From the host

```sh
cp /etc/rancher/k3s/k3s.yaml ~/.kube/config && sed -i "s,127.0.0.1,$(hostname -I | awk '{print $1}'),g" ~/.kube/config
```

```sh
docker run -p 4444:4444 \
    -p 5900:5900 \
    --shm-size="2g" \
    --network host \
    -v /home/ianpurton/Documents/bionic-gpt/crates/integration-testing/files:/workspace \
    selenium/standalone-chrome:latest
```

Make sure the Postgres port is open

## Open all the ports

```sh
cat <<EOF > open-ports.sh
# Push commands in the background, when the script exits, the commands will exit too
kubectl -n bionic-gpt port-forward --address 0.0.0.0 pod/bionic-db-cluster-1 5432 & \
kubectl -n bionic-gpt port-forward --address 0.0.0.0 deployment/mailhog 8025 & \

echo "Press CTRL-C to stop port forwarding and exit the script"
wait
EOF
chmod +x ./open-ports.sh
./open-ports.sh
rm ./open-ports.sh
```

## Open all the ports (minikube)

```sh
cat <<EOF > open-ports.sh
# Push commands in the background, when the script exits, the commands will exit too
minikube kubectl -- -n bionic-gpt port-forward --address 0.0.0.0 pod/bionic-db-cluster-1 5432 & \
minikube kubectl -- -n bionic-gpt port-forward --address 0.0.0.0 deployment/mailhog 8025 & \
DATABASE_URL=$(minikube kubectl -- get secret database-urls -n bionic-gpt -o jsonpath="{.data.migrations-url}" | base64 --decode | sed "s/bionic-db-cluster-rw/localhost/; s/\?sslmode=require//")
echo $DATABASE_URL
echo "Press CTRL-C to stop port forwarding and exit the script"
wait
EOF
chmod +x ./open-ports.sh
./open-ports.sh
rm ./open-ports.sh
```

## Run the tests

Get the host ip address

```sh
minikube ip
```

## Run

```sh
export DATABASE_URL=DATABASE_URL=postgres://db-owner:PASSSWORD@192.168.178.57:5432/bionic-gpt?sslmode=disable
export APPLICATION_URL=https://192.168.49.2
export WEB_DRIVER_URL=http://192.168.178.57:4444
export MAILHOG_URL=http://192.168.178.57:8025
cargo test
```