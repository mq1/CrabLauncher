-- SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
-- SPDX-License-Identifier: GPL-3.0-only

function Update(java_version)
    local os = get_os() == 'macos' and 'mac' or get_os()
    local arch = get_arch()

    local url = 'https://api.adoptium.net/v3/assets/latest/' .. java_version .. '/hotspot'
        .. '?architecture=' .. arch
        .. '&image_type=jre'
        .. '&os=' .. os
        .. '&vendor=eclipse'

    local assets = fetch_json(url)
    local archive_url = assets[1].package.link

    download_and_unpack(archive_url, 'runtimes/adoptium')
end
