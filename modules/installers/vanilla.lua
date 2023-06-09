-- SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
-- SPDX-License-Identifier: GPL-3.0-only

Info = {
    Name = 'Vanilla',

    -- https://pictogrammers.com/library/mdi/
    -- Optimized with https://jakearchibald.github.io/svgomg/
    IconSVG = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M4 2h16a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2m2 4v4h4v2H8v6h2v-2h4v2h2v-6h-2v-2h4V6h-4v4h-4V6H6Z"/></svg>'
}

DefaultRuntime = 'adoptium'

function GetVersions()
    local manifest = fetch_json('https://piston-meta.mojang.com/mc/game/version_manifest_v2.json')
    return manifest.versions
end

local function GetLibraries(version_manifest)
    local os = get_os()
    local arch = get_arch()
    local libraries = {}

    for library in version_manifest.libraries do
        local path = library.downloads.artifact.path

        -- filter out incompatible libraries
        if path.find('linux') and os ~= 'linux' then
            goto continue
        end

        if path.find('-windows-') and os ~= 'windows' then
            goto continue
        end

        if path.find('-macos-') and os ~= 'macos' then
            goto continue
        end

        if path.find('-unix-') and os ~= 'linux' and os ~= 'macos' then
            goto continue
        end

        if path.find('-x86.') and arch ~= 'x86_64' then
            goto continue
        end

        if path.find('-x86_64.') and arch ~= 'x86_64' then
            goto continue
        end

        if path.find('-arm64.') and arch ~= 'aarch64' then
            goto continue
        end

        if path.find('-aarch64.') and arch ~= 'aarch64' then
            goto continue
        end

        table.insert(libraries, library)

        ::continue::
    end

    return libraries
end

function Install(version)
    local manifest_path = 'versions/' .. version.id .. '.json'
    local manifest = download_json(version.url, manifest_path, version.sha1, 'sha1')

    -- Install assets
    local asset_index_path = 'assets/indexes/' .. manifest.assetIndex.id .. '.json'
    local asset_index = download_json(manifest.assetIndex.url, asset_index_path, manifest.assetIndex.sha1, 'sha1')

    for _, object in pairs(asset_index.objects) do
        local path = object.hash:sub(1, 2) .. '/' .. object.hash
        local url = 'https://resources.download.minecraft.net/' .. path
        download_file(url, 'assets/objects/' .. path, object.hash, 'sha1')
    end

    -- Install libraries
    local libraries = GetLibraries(manifest)
    for library in libraries do
        local url = library.downloads.artifact.url
        local path = 'libraries/' .. library.downloads.artifact.path
        download_file(url, path, library.downloads.artifact.sha1, 'sha1')
    end

    -- Install client
    local client_path = 'libraries/com/mojang/minecraft' .. manifest.id .. '/' .. 'minecraft-' .. manifest.id .. '.jar'
    local client_url = manifest.downloads.client.url
    download_file(client_url, client_path, manifest.downloads.client.sha1, 'sha1')

    -- Update runtime
    UpdateRuntime(manifest.javaVersion.majorVersion)
end
