# Rustix

Sistema operacional bare-metal em Rust construído para fins educacionais, focado em entender os fundamentos e funções de um kernel e como implementá-las do zero.

## Requisitos

- Toolchain Rust `nightly` com `x86_64-unknown-none` e `rust-src`:
  ```bash
  rustup toolchain install nightly
  rustup component add rust-src --toolchain nightly
  rustup target add x86_64-unknown-none --toolchain nightly
  ```
- QEMU (`qemu-system-x86_64`)
- `make`
- Firmware OVMF, necessário apenas para boot UEFI:
  - Fedora: `sudo dnf install edk2-ovmf`
  - Debian/Ubuntu: `sudo apt install ovmf`

## Estrutura

- `src/` — o kernel propriamente dito, compilado para `x86_64-unknown-none`.
- `builder/` — ferramenta que roda no host: compila o kernel e empacota o binário resultante em imagens de disco inicializáveis usando o crate `bootloader` 0.11.
- `Makefile` — automatiza build e execução no QEMU.

O `.cargo/config.toml` define `x86_64-unknown-none` como target padrão do workspace. Por isso o `builder` precisa ser compilado com `--target x86_64-unknown-linux-gnu` explícito: ele é um programa de host, não parte do kernel.

## Uso

| Comando | O que faz |
| --- | --- |
| `make build` | Compila o kernel e gera `boot-bios.img` e `boot-uefi.img` |
| `make run` | Faz o build e inicia a imagem BIOS no QEMU |
| `make run-uefi` | Faz o build e inicia a imagem UEFI no QEMU com firmware OVMF |
| `make debug` | Igual ao `run`, mas com a CPU parada esperando GDB na porta 1234 |
| `make debug-uefi` | Igual ao `run-uefi`, mas parado esperando GDB na porta 1234 |
| `make gdb` | Conecta o GDB a uma sessão iniciada por `make debug` |
| `make clean` | Remove os artefatos de build do kernel e do builder |
| `make help` | Lista os alvos disponíveis |

As imagens são geradas em `target/x86_64-unknown-none/debug/`.

### Variáveis de override

Todas podem ser passadas na linha de comando:

```bash
make run QEMU_EXTRA="-d int,cpu_reset"   # flags extras do QEMU
make run QEMU_MEM=256M                   # memória da máquina virtual
make run-uefi OVMF_CODE=/caminho/OVMF_CODE.fd OVMF_VARS=/caminho/OVMF_VARS.fd
```

Toda execução usa `-serial stdio`, então a saída da porta serial do kernel aparece direto no terminal, e `-no-reboot`, para que um triple fault pare a máquina virtual em vez de reiniciá-la em loop.

### Depuração com GDB

Em um terminal:

```bash
make debug
```

Em outro:

```bash
make gdb
```

Os símbolos vêm do ELF do kernel em `target/x86_64-unknown-none/debug/rustix`, não da imagem de disco.

## Sobre o firmware UEFI

O OVMF moderno é distribuído em dois arquivos: `OVMF_CODE.fd`, que é o firmware em si e é montado somente leitura, e `OVMF_VARS.fd`, que é o armazenamento de variáveis NVRAM e precisa ser gravável. Por isso os dois são anexados como dispositivos `pflash` separados, em vez de usar a opção `-bios`.

O Makefile procura esses arquivos nos caminhos usados pelas distribuições mais comuns e copia o `OVMF_VARS.fd` para o diretório de build antes de usá-lo, de modo que o arquivo do sistema nunca é modificado.

## Execução manual

O Makefile é só conveniência. Os comandos equivalentes são:

```bash
# Compilar e gerar as imagens
cargo +nightly run --manifest-path builder/Cargo.toml --target x86_64-unknown-linux-gnu

# BIOS
qemu-system-x86_64 -m 128M -serial stdio -no-reboot \
  -drive format=raw,file=target/x86_64-unknown-none/debug/boot-bios.img

# UEFI
cp /usr/share/OVMF/OVMF_VARS.fd target/x86_64-unknown-none/debug/OVMF_VARS.fd
chmod u+w target/x86_64-unknown-none/debug/OVMF_VARS.fd
qemu-system-x86_64 -m 128M -serial stdio -no-reboot \
  -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE.fd \
  -drive if=pflash,format=raw,file=target/x86_64-unknown-none/debug/OVMF_VARS.fd \
  -drive format=raw,file=target/x86_64-unknown-none/debug/boot-uefi.img
```

## Estado atual

O kernel é um binário freestanding (`#![no_std]`, `#![no_main]`) que recebe o `BootInfo` do bootloader e monta sua própria GDT (`src/gdt.rs`): null descriptor, code descriptor e data descriptor, carregados via `lgdt` e um far-return para recarregar `CS` em modo longo. Panics são reportados pela porta serial COM1 (`0x3F8`) via UART 16550, com arquivo, linha, coluna e mensagem. Ainda não há IDT, paginação própria nem alocador de heap.

O foco de desenvolvimento é o caminho UEFI. O caminho BIOS continua sendo gerado e é útil como segundo alvo de teste: uma falha que aparece nos dois aponta para o kernel, enquanto uma falha que aparece em apenas um aponta para a interface de boot.
