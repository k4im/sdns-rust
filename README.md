
# SDNS

Software de command-line utilizando para realizar buscas rapidas de registros de DNS para um determinado dominio.

Através deste software é possivel estar realizando buscas de registros de DNS para um dominio, sendo possivel estar filtrando registros MX, TXT, IPV4 e NS.



## Autor

- [@k4im](https://www.github.com/k4im)


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

