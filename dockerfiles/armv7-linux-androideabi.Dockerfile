# This Docker image allows us to use 'x86_64-pc-windows-gnu' toolchain
# while also installing LLVM, which is required by 'gdnative-sys' crate.

FROM rustembedded/cross:armv7-linux-androideabi

RUN apt-get update && \
    apt-get install clang-format clang-tidy clang libc++-dev libc++1 libc++abi-dev libc++abi1 libclang-dev libclang1 libomp-dev libomp5 lldb llvm-dev llvm-runtime llvm -y

RUN apt-get install wget -y

RUN apt-get install lsb-release -y

RUN apt-get install software-properties-common -y

RUN apt-get install apt-transport-https ca-certificates -y

RUN apt-get install g++-multilib libc6-dev-i386 -y

RUN apt-get update

RUN apt-get install default-jre -y

RUN java -version

RUN apt-get install default-jdk -y

RUN javac -version

RUN bash -c "$(wget -O - https://apt.llvm.org/llvm.sh)"