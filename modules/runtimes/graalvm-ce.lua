-- SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
-- SPDX-License-Identifier: GPL-3.0-only

local function GetValidAsset(assets, java_version)
    local os = get_os()
    os = os == 'macos' and 'darwin' or os

    local arch = get_arch()

    local valid_asset = nil
    for asset in assets do
        if not asset.name.find('graalvm') then
            goto continue
        end

        if not asset.name.find('java' .. java_version) then
            goto continue
        end

        if not asset.name.find(os) then
            goto continue
        end

        if not asset.name.find(arch) then
            goto continue
        end

        valid_asset = asset
        break

        ::continue::
    end

    return valid_asset
end

function Update(java_version)
    local latest_release = fetch_json('https://api.github.com/repos/graalvm/graalvm-ce-builds/releases/latest')
    local asset = GetValidAsset(latest_release.assets, java_version)

    if asset then
        download_and_unpack(asset.browser_download_url, 'runtimes/graalvm-ce')
        return
    end

    -- download dev version if no release is found
    local dev_release = fetch_json('https://api.github.com/repos/graalvm/graalvm-ce-dev-builds/releases/latest')
    local dev_asset = GetValidAsset(dev_release.assets, java_version)

    if dev_asset then
        download_and_unpack(dev_asset.browser_download_url, 'runtimes/graalvm-ce')
    else
        error('No GraalVM release found for Java ' .. java_version)
    end
end
