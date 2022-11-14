#
# Contains the public keys for users that should automatically be granted access
# to ALL testnets and datacenter nodes.
#
# To add an entry into this list:
# 1. Run: ssh-keygen -t ecdsa -N '' -f ~/.ssh/id-miraland-testnet
# 2. Add an entry to MIRALAND_USERS with your username
# 3. Add an entry to MIRALAND_PUBKEYS with the contents of ~/.ssh/id-miraland-testnet.pub
#
# If you need multiple keys with your username, repeatedly add your username to MIRALAND_USERS, once per key
#

MIRALAND_USERS=()
MIRALAND_PUBKEYS=()

MIRALAND_USERS+=('miracle17')
MIRALAND_PUBKEYS+=('ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBO1huBAwhQ6dqw9oViYcRUriNftB64dKj+Lj8QnozotMQPIg+ubYHxzlQro+mvcYqblPQ7HwLt0R6EVRvgrllWk= miracle17@solarti.io')
MIRALAND_USERS+=('miracle17')
MIRALAND_PUBKEYS+=('ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIJ4ocIWgGhaR1yTNgGK9bxSKkPkWcMRYQBucUeT+6Muy miracle17@solarti.io')
MIRALAND_USERS+=('miracle17')
MIRALAND_PUBKEYS+=('ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIFkaw6BkPqNqoaTdzSdbWGFghqjQAC46Zk9jdqIn+COc miracle17@Miraland-MacBook-Pro.local')
MIRALAND_USERS+=('miracle17')
MIRALAND_PUBKEYS+=('ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMG6g19Bic33Ogouu79bgvbNTVYFB0a4ZyWU5+Jtdss7 miracle17@miraland.top')
MIRALAND_USERS+=('miracle17')
MIRALAND_PUBKEYS+=('ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAMXHTFwayqA7MkMSDMIXv0AfuExsK0EKyYJ2ptHzD6j miracle17@miraland.xyz')

#ci-devnet-mln-deployer-1 (buildkite-agent@arcaps-dev-01)
MIRALAND_USERS+=('buildkite-agent')
MIRALAND_PUBKEYS+=('ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBCp1lfkcmpD5L0V4riUiLzPwlStk0s1G3HzjHDGmR/qwWE0exUv5/1RtpKeGXPUHB8Xrz+fLEnt1CO3icnuPgdc= buildkite-agent@arcaps-dev-01')
#ci-devnet-mln-deployer-2 (buildkie-agent@miraland-dev-01)
MIRALAND_USERS+=('buildkite-agent')
MIRALAND_PUBKEYS+=('ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBJlQWfZsEnnbpIB4AcHD54MXV4lLhy0S4EOaJOKeX8kbfDB56UjvSj+udY8aeKIIE4yhRyO3OUIk+gJ1QaaNRL4= buildkite-agent@miraland-dev-01')
