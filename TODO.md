```c
bool C3D_Init(size_t cmdBufSize)
{
	int i;
	C3D_Context* ctx = C3Di_GetContext();

	if (ctx->flags & C3DiF_Active)
		return false;

	cmdBufSize = (cmdBufSize + 0xF) &~ 0xF; // 0x10-byte align
	ctx->cmdBufSize = cmdBufSize/4;
	ctx->cmdBuf = (u32*)linearAlloc(cmdBufSize);
	ctx->cmdBufUsage = 0;
	if (!ctx->cmdBuf)
		return false;

	ctx->gxQueue.maxEntries = 32;
	ctx->gxQueue.entries = (gxCmdEntry_s*)malloc(ctx->gxQueue.maxEntries*sizeof(gxCmdEntry_s));
	if (!ctx->gxQueue.entries)
	{
		linearFree(ctx->cmdBuf);
		return false;
	}

	ctx->flags = C3DiF_Active | C3DiF_TexEnvBuf | C3DiF_TexEnvAll | C3DiF_Effect | C3DiF_TexStatus | C3DiF_TexAll;

	// TODO: replace with direct struct access
	C3D_DepthMap(true, -1.0f, 0.0f);
	C3D_CullFace(GPU_CULL_BACK_CCW);
	C3D_StencilTest(false, GPU_ALWAYS, 0x00, 0xFF, 0x00);
	C3D_StencilOp(GPU_STENCIL_KEEP, GPU_STENCIL_KEEP, GPU_STENCIL_KEEP);
	C3D_BlendingColor(0);
	C3D_EarlyDepthTest(false, GPU_EARLYDEPTH_GREATER, 0);
	C3D_DepthTest(true, GPU_GREATER, GPU_WRITE_ALL);
	C3D_AlphaTest(false, GPU_ALWAYS, 0x00);
	C3D_AlphaBlend(GPU_BLEND_ADD, GPU_BLEND_ADD, GPU_SRC_ALPHA, GPU_ONE_MINUS_SRC_ALPHA, GPU_SRC_ALPHA, GPU_ONE_MINUS_SRC_ALPHA);
	C3D_FragOpMode(GPU_FRAGOPMODE_GL);
	C3D_FragOpShadow(0.0, 1.0);

	ctx->texConfig = BIT(12);
	ctx->texShadow = BIT(0);
	ctx->texEnvBuf = 0;
	ctx->texEnvBufClr = 0xFFFFFFFF;
	ctx->fogClr = 0;
	ctx->fogLut = NULL;

	for (i = 0; i < 3; i ++)
		ctx->tex[i] = NULL;

	for (i = 0; i < 6; i ++)
		C3D_TexEnvInit(&ctx->texEnv[i]);

	ctx->fixedAttribDirty = 0;
	ctx->fixedAttribEverDirty = 0;

	C3Di_RenderQueueInit();
	aptHook(&hookCookie, C3Di_AptEventHook, NULL);

	return true;
}
```

Turn everything above into Encodable Commands, i guess?

Perhaps something like this:

```rust
let mut enc = todo!();

enc += DepthMap(-1.0,0.0)
  + CullFace(CullFace::BACK_CCW)
  - StencilTest::default()
  + StencilOp::default()
  + BlendingColor(0)
  - EarlyDepthTest::default()
  + DepthTest(DepthTest::GREATER,DepthTest::WRITE_ALL)
  - AlphaTest::default()
  + AlphaBlend(AlphaBlend::ADD,AlphaBlend::ADD,AlphaBlend::SRC_ALPHA,AlphaBlend::ONE_MINUS_SRC_ALPHA,AlphaBlend::SRC_ALPHA,AlphaBlend::ONE_MINUS_SRC_ALPHA)
```

Also, move the command buffer into the Queue, so that it can Drop it when it receives the Event that it's looking for.

Perhaps set up an IRQ?
