# Boing Boing Boing
Audio DSP library for musical applications, written in Rust.

## Roadmap (2023)
Boing3 is still in very early development. The goal right
now is to get it working with a similar functionality and
scope to sndkit. This includes the set of DSP algorithms,
the graforge library for constructing sounds, and a API
layer that sits on top of graforge and the DSP.

- [ ] Port initial set of sndkit DSP algorithms
- [ ] Port graforge functionality
- [ ] Port sndkit API functionality

Note that there are no plans to port LIL (the embedded
scripting language included in sndkit), or have any
integrated scripting language included in Boing3. Any such
bindings will be declared out of scope.

### DSP algorithms
- [ ] osc
- [ ] fmpair
- [ ] rline
- [ ] peakeq
- [x] bigverb
- [ ] dcblocker
- [ ] vardelay
- [ ] phasewarp
- [ ] modalres
- [ ] bitnoise
- [ ] chaosnoise
- [ ] oscf
- [ ] bezier
- [ ] expmap
- [ ] phsclk
- [ ] phasor
- [ ] biramp
- [ ] scale
- [ ] rephasor
- [ ] smoother
- [ ] metro
- [ ] expon
- [ ] chorus
- [ ] bitosc
- [ ] env
- [ ] glottis
- [ ] tract
- [x] blep
- [ ] vowel
- [ ] vowshape
- [ ] clkphs
- [x] butterworth
- [ ] sparse
- [ ] tseq
- [ ] adsr
- [ ] tgate
- [ ] tblin
- [ ] trand
- [ ] crossfade
- [ ] tsmp
- [ ] qgliss
- [ ] tdiv
- [ ] shelf
- [ ] lpf
- [ ] envar
- [ ] euclid
- [ ] gtick

### Graforge API
- [ ] buffer
- [ ] buffer pool
- [ ] stack
- [ ] cable
- [ ] node
- [ ] patch
- [ ] subpatch

### sndkit core API
- [ ] compute block
- [ ] compute seconds
- [ ] stack push/pop
- [ ] RNG
