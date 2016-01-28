initSidebarItems({"struct":[["Sampler","Sampler component. - init: `&str` = name of the sampler - data: `Sampler`"],["ShaderResource","Shader resource component (SRV). Typically is a view into some texture, but can also be a buffer. - init: `&str` = name of the resource - data: `ShaderResourceView<T>`"],["TextureSampler","A convenience type for a texture paired with a sampler. It only makes sense for DX9 class hardware, where every texture by default is bundled with a sampler, hence they are represented by the same name. In DX10 and higher samplers are totally separated from the textures. - init: `&str` = name of the sampler/texture (assuming they match) - data: (`ShaderResourceView<T>`, `Sampler`)"],["UnorderedAccess","Unordered access component (UAV). A writable resource (texture/buffer) with no defined access order across simultaneously executing shaders. Supported on DX10 and higher. - init: `&str` = name of the resource - data: `UnorderedAccessView<T>`"]]});