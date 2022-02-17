unsigned int get_this_ptr()
{
    unsigned int this = 0;

    __asm
    {
        mov this, ecx
    }

    return this;
}

void set_this_ptr(unsigned int this)
{
    __asm
    {
        mov ecx, this
    }
}
