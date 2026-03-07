package com.justin13888.pixles.utils

/**
 * Pure-Kotlin little-endian implementation of xxHash-64.
 * Sufficient for file deduplication purposes.
 *
 * Reference: https://github.com/Cyan4973/xxHash/blob/dev/doc/xxhash_spec.md
 */
@OptIn(ExperimentalUnsignedTypes::class)
object XxHash64 {
    private val PRIME1 = 0x9E3779B185EBCA87UL
    private val PRIME2 = 0xC2B2AE3D27D4EB4FUL
    private val PRIME3 = 0x165667B19E3779F9UL
    private val PRIME4 = 0x85EBCA77C2B2AE63UL
    private val PRIME5 = 0x27D4EB2F165667C5UL

    fun hash(input: ByteArray, seed: ULong = 0UL): ULong {
        var pos = 0
        val len = input.size
        val h64: ULong

        if (len >= 32) {
            val limit = len - 32
            var v1 = seed + PRIME1 + PRIME2
            var v2 = seed + PRIME2
            var v3 = seed + 0UL
            var v4 = seed - PRIME1

            do {
                v1 = round(v1, getLong(input, pos)); pos += 8
                v2 = round(v2, getLong(input, pos)); pos += 8
                v3 = round(v3, getLong(input, pos)); pos += 8
                v4 = round(v4, getLong(input, pos)); pos += 8
            } while (pos <= limit)

            var acc = rotl64(v1, 1) + rotl64(v2, 7) + rotl64(v3, 12) + rotl64(v4, 18)
            acc = mergeRound(acc, v1)
            acc = mergeRound(acc, v2)
            acc = mergeRound(acc, v3)
            acc = mergeRound(acc, v4)
            h64 = acc
        } else {
            h64 = seed + PRIME5
        }

        var h = h64 + len.toULong()

        while (pos + 8 <= len) {
            val k1 = round(0UL, getLong(input, pos))
            h = h xor k1
            h = rotl64(h, 27) * PRIME1 + PRIME4
            pos += 8
        }

        if (pos + 4 <= len) {
            h = h xor (getInt(input, pos).toULong() * PRIME1)
            h = rotl64(h, 23) * PRIME2 + PRIME3
            pos += 4
        }

        while (pos < len) {
            h = h xor (input[pos].toUByte().toULong() * PRIME5)
            h = rotl64(h, 11) * PRIME1
            pos++
        }

        return avalanche(h)
    }

    private fun round(acc: ULong, input: ULong): ULong {
        var a = acc + (input * PRIME2)
        a = rotl64(a, 31)
        return a * PRIME1
    }

    private fun mergeRound(acc: ULong, v: ULong): ULong {
        val v2 = round(0UL, v)
        return (acc xor v2) * PRIME1 + PRIME4
    }

    private fun avalanche(h: ULong): ULong {
        var x = h xor (h shr 33)
        x *= PRIME2
        x = x xor (x shr 29)
        x *= PRIME3
        x = x xor (x shr 32)
        return x
    }

    private fun rotl64(value: ULong, shift: Int): ULong =
        (value shl shift) or (value shr (64 - shift))

    private fun getLong(array: ByteArray, pos: Int): ULong =
        array[pos].toUByte().toULong() or
            (array[pos + 1].toUByte().toULong() shl 8) or
            (array[pos + 2].toUByte().toULong() shl 16) or
            (array[pos + 3].toUByte().toULong() shl 24) or
            (array[pos + 4].toUByte().toULong() shl 32) or
            (array[pos + 5].toUByte().toULong() shl 40) or
            (array[pos + 6].toUByte().toULong() shl 48) or
            (array[pos + 7].toUByte().toULong() shl 56)

    private fun getInt(array: ByteArray, pos: Int): UInt =
        array[pos].toUByte().toUInt() or
            (array[pos + 1].toUByte().toUInt() shl 8) or
            (array[pos + 2].toUByte().toUInt() shl 16) or
            (array[pos + 3].toUByte().toUInt() shl 24)
}
