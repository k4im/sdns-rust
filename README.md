
# SDNS

Software de command-line utilizando para realizar buscas rapidas de registros de DNS para um determinado dominio.

Através deste software é possivel estar realizando buscas de registros de DNS para um dominio, sendo possivel estar filtrando registros MX, TXT, IPV4 e NS.



## Autor

- [@k4im](https://www.github.com/k4im)

## Exemplo
![Peek 2024-07-12 10-36](https://github.com/user-attachments/assets/704173f6-b928-40c9-b4eb-9a666be5b581)


## Releases
Para baixar diretamente o binario para linux:

[Linux-Version](https://github.com/k4im/sdns-rust/releases/tag/v1.0.0)

## Build

Para buildar o projeto rode o comando abaixo:

```bash
cargo b -q -r
```

## Install

Para instalar o software execute o seguinte comando:

``` bash
sudo mv target/release/sdns /usr/bin/
```

## Comandos

Filtrar todos os registros `MX`, `NS`, `TXT`, `IPV4`:

``` bash
sdns exemplo.com
```


Filtrar apenas registros `NS`:

``` bash
sdns -n exemplo.com
```

``` bash
sdns --name-server exemplo.com
```

Filtrar apenas registros  `MX`:

``` bash
sdns -m exemplo.com
```

``` bash
sdns --mx exemplo.com
```

Filtrar apenas registros  `IPV4`:

``` bash
sdns -i exemplo.com
```

``` bash
sdns --ipv4 exemplo.com
```


Filtrar apenas registros  `TXT`:

``` bash
sdns -t exemplo.com
```

``` bash
sdns --txt exemplo.com
```


