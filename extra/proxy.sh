#!/bin/sh
set -eux

# This script is written to be as POSIX as possible
# so it works fine for all Unix-like operating systems

test_cmd() {
  command -v "$1" >/dev/null
}

# proxy version
catalyst_new_ver="${1}"
# proxy directory
# eval to resolve '~' into proper user dir
eval catalyst_dir="'${2}'"

case "${catalyst_new_ver}" in
  v*)
    catalyst_new_version=$(echo "${catalyst_new_ver}" | cut -d'v' -f2)
    catalyst_new_ver_tag="${catalyst_new_ver}"
  ;;
  nightly*)
    catalyst_new_version="${catalyst_new_ver}"
    catalyst_new_ver_tag=$(echo ${catalyst_new_ver} | cut -d '-' -f1)
  ;;
  *)
    printf 'Unknown version\n'
    exit 1
  ;;
esac

if [ -e "${catalyst_dir}/catalyst" ]; then
  catalyst_installed_ver=$("${catalyst_dir}/catalyst" --version | cut -d' ' -f2)

  printf '[DEBUG]: Current proxy version: %s\n' "${catalyst_installed_ver}"
  printf '[DEBUG]: New proxy version: %s\n' "${catalyst_new_version}"
  if [ "${catalyst_installed_ver}" = "${catalyst_new_version}" ]; then
    printf 'Proxy already exists\n'
    exit 0
  else
    printf 'Proxy outdated. Replacing proxy\n'
    rm "${catalyst_dir}/catalyst"
  fi
fi

for _cmd in tar gzip uname; do
  if ! test_cmd "${_cmd}"; then
    printf 'Missing required command: %s\n' "${_cmd}"
    exit 1
  fi
done

# Currently only linux/darwin are supported
case $(uname -s) in
  Linux) os_name=linux ;;
  Darwin) os_name=darwin ;;
  *)
    printf '[ERROR] unsupported os\n'
    exit 1
  ;;
esac

# Currently only amd64/arm64 are supported
case $(uname -m) in
  x86_64|amd64|x64) arch_name=x86_64 ;;
  arm64|aarch64) arch_name=aarch64 ;;
  # riscv64) arch_name=riscv64 ;;
  *)
    printf '[ERROR] unsupported arch\n'
    exit 1
  ;;
esac

catalyst_download_url="https://github.com/catalyst/catalyst/releases/download/${catalyst_new_ver_tag}/catalyst-proxy-${os_name}-${arch_name}.gz"

printf 'Creating "%s"\n' "${catalyst_dir}"
mkdir -p "${catalyst_dir}"
cd "${catalyst_dir}"

if test_cmd 'curl'; then
  # How old curl has these options? we'll find out
  printf 'Downloading using curl\n'
  curl --proto '=https' --tlsv1.2 -LfS -O "${catalyst_download_url}"
  # curl --proto '=https' --tlsv1.2 -LZfS -o "${tmp_dir}/catalyst-proxy-${os_name}-${arch_name}.gz" "${catalyst_download_url}"
elif test_cmd 'wget'; then
  printf 'Downloading using wget\n'
  wget "${catalyst_download_url}"
else
  printf 'curl/wget not found, failed to download proxy\n'
  exit 1
fi

printf 'Decompressing gzip\n'
gzip -df "${catalyst_dir}/catalyst-proxy-${os_name}-${arch_name}.gz"

printf 'Renaming proxy \n'
mv -v "${catalyst_dir}/catalyst-proxy-${os_name}-${arch_name}" "${catalyst_dir}/catalyst"

printf 'Making it executable\n'
chmod +x "${catalyst_dir}/catalyst"

printf 'catalyst-proxy installed\n'

exit 0
