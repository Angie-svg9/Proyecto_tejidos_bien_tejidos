use anchor_lang::prelude::*;
declare_id!("BavuxHPkdt66ArWDT93jezxBqzvErwqw2DmfZZoJZY7B");

#[program] 
pub mod tienda_crochet { // Tienda es como a se llama el archivo en el programa 
    use super::*;

    // Para crear la tienda y guardamos el nombre y el id del comprador
    pub fn crear_tienda(context: Context<NuevaTienda>, nombre: String) -> Result<()> {
       
        let comprador_id = context.accounts.comprador.key();
        msg!("Comprador_id: {}", comprador_id);

        let hilos: Vec<Hilo> = Vec::new(); // Es la lista de Hilo vacía 

        
        context.accounts.tienda.set_inner(Tienda { 
            comprador: comprador_id,
            nombre,
            hilos,
            
        });
        Ok(()) 
    }

   // Agregar un nuevo hilo a la lista y solo lo puede hacer el comprador
    pub fn agregar_hilo(context: Context<NuevoHilo>, nombre: String, color: String, grosor: u16) -> Result<()> {
        require!( 
            context.accounts.tienda.comprador == context.accounts.comprador.key(), 
            Errores::NoEresElComprador 
        ); 

        let nuevo_hilo = Hilo { 
            nombre,
            color,
            grosor,
            disponible: true,
        };

        context.accounts.tienda.hilos.push(nuevo_hilo); 

        Ok(()) 
    }

    // Eliminar un hilo de la lista 
    pub fn eliminar_hilo(context: Context<NuevoHilo>, nombre: String) -> Result<()> {
        require!(
            context.accounts.tienda.comprador == context.accounts.comprador.key(),
            Errores::NoEresElComprador
        );
        // Buscar en la lista el hilo para saber que si sí está 
        let hilos = &mut context.accounts.tienda.hilos;
        for i in 0..hilos.len() {
            if hilos[i].nombre == nombre { 
                hilos.remove(i); // Para quitar de la lista 
                msg!("Hilo {} eliminado!", nombre); // Para que mande un mensaje
                return Ok(()); 
            }
        }
        Err(Errores::HiloNoExiste.into()) // Por si el hilo no está mande un mensaje de error
    }

   // Para ver la lista de los hilos 
    pub fn ver_hilos(context: Context<NuevoHilo>) -> Result<()> {
        require!( 
            context.accounts.tienda.comprador == context.accounts.comprador.key(),
            Errores::NoEresElComprador // Error para cuando no es el comprador
        );

       
        msg!("La lista de hilos actualmente es: {:#?}", context.accounts.tienda.hilos);
        Ok(()) 
    }

    
    // Para saber si el hilo está disponible o no
    pub fn alternar_hilo(context: Context<NuevoHilo>, nombre: String) -> Result<()> {
        require!( 
            context.accounts.tienda.comprador == context.accounts.comprador.key(),
            Errores::NoEresElComprador // Si no es el comprador marca error
        );

        let hilos = &mut context.accounts.tienda.hilos; 
        for i in 0..hilos.len() { 
            let estado = hilos[i].disponible;
            if hilos[i].nombre == nombre {
                let nuevo_estado = !estado;
                hilos[i].disponible = nuevo_estado;
                msg!("El hilo: {} ahora esta disponible: {}", nombre, nuevo_estado);
                return Ok(()); // Transaccion exitosa
            }
        }

        Err(Errores::HiloNoExiste.into()) // Por si el hilo no existe mande error
    }

}


#[error_code]
pub enum Errores {
    #[msg("Error, no eres el comprador del hilo que deseas modificar")]
    NoEresElComprador,
    #[msg("Error, el hilo con el que deseas interactuar no existe")]
    HiloNoExiste,
}

#[account] 
#[derive(InitSpace)]
pub struct Tienda { 
    pub comprador: Pubkey, 

    #[max_len(60)] 
   pub  nombre: String,

    #[max_len(10)] 
   pub  hilos: Vec<Hilo>,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Hilo {
   #[max_len(60)]  // Cantidad maxima de caracteres para nombre 
    pub nombre: String,
    #[max_len(30)]  // Cantidad maxima de caracteres para color
    pub color: String, 
    pub grosor: u16, 
    pub disponible: bool,
}



#[derive(Accounts)]
pub struct NuevaTienda<'info> {
    #[account(mut)] 
    pub comprador: Signer<'info>,

    #[account(
        init, // Inidica que al llamar la instruccuion se creara una cuenta
        // puede ser remplazado por "init_if_needed" para que solo se cree una vez por caller
        payer = comprador, // Se especifica que quien paga el llamado a la instruccion, en este caso llama la instruccion 
        space = Tienda::INIT_SPACE + 8, // Se calcula el espacio requerido para almacenar el Solana Program On-Chain
        seeds = [b"tienda", comprador.key().as_ref()], // Se especifica que la cuenta es una PDA que depende de un string y el id del comprador
        bump // Metodo para determinar el el id de la tienda  en base a lo anterior 
    )]
    pub tienda: Account<'info, Tienda>, 

    pub system_program: Program<'info, System>, 
}
#[derive(Accounts)]
pub struct NuevoHilo<'info> {
    pub comprador: Signer<'info>,

    #[account(mut)] 
    pub tienda: Account<'info, Tienda>, 
}
