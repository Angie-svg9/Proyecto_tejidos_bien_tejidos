use anchor_lang::prelude::*;

declare_id!("EPpa5cEUVDphU7brnGo2DEHwXRjCMKxm35Tjq9hgRbhL");

#[program]
pub mod tiendapeliculas {
    use super::*;

    pub fn crear_tienda(ctx: Context<Nuevatienda>, nombre: String) -> Result<()> {
        let tienda = &mut ctx.accounts.tienda;
        tienda.owner = ctx.accounts.owner.key();
        tienda.nombre = nombre;
        tienda.peliculas = Vec::new(); // Inicializa el vector vacío
        Ok(())
    }

    pub fn agregar_pelicula(
        ctx: Context<OperarTienda>, 
        titulo: String,      
        descripcion: String, 
        anio: u64,        
        director: String,    
        genero: String,
        precio_sol: u64,
        precio_usdc: u64, // Agregamos opción de pago en USDC
    ) -> Result<()> {
        let tienda = &mut ctx.accounts.tienda;
        
        // Validación de seguridad
        require!(tienda.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let nueva_peli = Pelicula {
            titulo,
            descripcion,
            anio,
            director,
            genero,
            precio_sol,
            precio_usdc,
            disponible: true,
        };

        tienda.peliculas.push(nueva_peli);
        Ok(())
    }

    pub fn alternar_estado(ctx: Context<OperarTienda>, titulo: String) -> Result<()> {
        let tienda = &mut ctx.accounts.tienda;
        require!(tienda.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        // Buscamos por título
        let peli = tienda.peliculas.iter_mut().find(|p| p.titulo == titulo);
        
        match peli {
            Some(p) => {
                p.disponible = !p.disponible;
                msg!("Estado de {} cambiado a {}", titulo, p.disponible);
                Ok(())
            },
            None => Err(Errores::Noexisteestapelicula.into()),
        }
    }
}

#[account]
#[derive(InitSpace)]
pub struct Tiendapeliculas {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre: String,
    #[max_len(10)] // Limitamos a 10 películas para no exceder el tamaño de cuenta
    pub peliculas: Vec<Pelicula>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Pelicula {
    #[max_len(40)]
    pub titulo: String,      
    #[max_len(100)]
    pub descripcion: String, 
    pub anio: u64,    
    #[max_len(40)]
    pub director: String,    
    #[max_len(20)]
    pub genero: String,
    pub precio_sol: u64,
    pub precio_usdc: u64,
    pub disponible: bool,
}

#[derive(Accounts)]
pub struct Nuevatienda<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + Tiendapeliculas::INIT_SPACE,
        seeds = [b"tienda", owner.key().as_ref()],
        bump
    )]
    pub tienda: Account<'info, Tiendapeliculas>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OperarTienda<'info> {
    #[account(mut)]
    pub tienda: Account<'info, Tiendapeliculas>,
    pub owner: Signer<'info>,
}

#[error_code]
pub enum Errores {
    #[msg("No eres el propietario de esta tienda")]
    NoEresElOwner,
    #[msg("La pelicula buscada no existe")]
    Noexisteestapelicula,
}
