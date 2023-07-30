use bitcoin::messages::read_from_bytes::{decode_hex, read_string_from_bytes, encode_hex};
use bitcoin::wallet_utils::broadcast_txn::BroadcastTxn;
use bitcoin::wallet_utils::get_proof::GetProof;
use bitcoin::wallet_utils::merkle_block::MerkleBlock;
use gtk::{prelude::*, Widget, Fixed, CssProvider, Entry, Label, Clipboard, ListStore, SpinButton};
use gtk::{Dialog, Image};
use gtk::{Builder,Button,Box};
use gtk::Window;
use std::io::Write;
use std::net::TcpStream;
use std::sync::Mutex;
use std::sync::Arc;
use crate::accounts::Accounts;
use crate::proof_of_inclusion::get_proof_of_inclusion::*;
use crate::interface_error::InterfaceError;
use crate::transactions::create_transaction_error::TransactionCreateError;
use crate::transactions::create_transactions::{create_transaction, is_string_bech32, address_from_pubkey};
use crate::views::transaction_tree_view::create_transaction_tree_view;
use crate::views::transaction_view::create_transaction_view;
use super::handle_styles::set_button_style;
use super::handler_constants::*;

/// Sets various buttons on the interface.
///
/// # Arguments
///
/// * `builder` - A reference to the builder object.
/// * `accounts` - A shared mutable reference to the accounts.
/// * `node` - A shared mutable reference to the TCP stream node.
/// * `store` - A reference to the list store.
///
/// # Errors
///
/// Returns an `InterfaceError` if an error occurs while setting the buttons.
pub fn set_buttons(builder:&Builder, accounts: Arc<Mutex<Accounts>>, node: Arc<Mutex<TcpStream>>, store: &ListStore) -> Result<(), InterfaceError>{
    set_login_button(builder, accounts.clone())?;
    set_overview_button(builder)?;
    set_send_button(builder)?;
    set_receive_button(builder)?;
    set_transactions_button(builder, store)?;
    set_add_recipient_button(builder)?;
    set_clear_all_button(builder)?;
    set_copy_button(builder)?;
    set_poi_button(builder)?;
    set_send_transaction_button(builder, node.clone(), accounts)?;
    set_make_proof_button(builder, node)?;
    set_new_account_button(builder)?;
    set_return_button(builder)?;
    set_ok_button(builder)?;
    //set_combo_box_button(builder, accounts)?;
    Ok(())
}

/// Replaces the content of a GTK container with a new widget.
///
/// # Arguments
///
/// * `container` - A reference to the GTK container (`Box`) whose content will be replaced.
/// * `widget` - A reference to the new GTK widget that will replace the existing content.
///
/// # Description
///
/// This function replaces the current content of a GTK container with a new widget. It removes the last widget from the container (if any) and adds the new widget in its place. The new widget is then expanded horizontally and vertically within the container.
///
fn replace_content(container: &Box,widget: &gtk::Widget) {
    let children = container.get_children();
    if let Some(last_widget) = children.last() {
        container.remove(last_widget);
        container.add(widget);
        widget.set_hexpand(true);
        widget.set_vexpand(true);
    }
}

/// Sets up the overview button in the user interface.
///
/// # Arguments
///
/// * `builder` - A reference to the GTK GUI builder.
///
/// # Returns
///
/// Returns `Result<(), InterfaceError>` indicating the success or failure of setting up the overview button. An `Ok` value is returned upon successful setup, while an `Err` value of `InterfaceError` type is returned in case of any errors.
///
/// # Description
///
/// This function sets up the overview button in the user interface. It retrieves the overview button, overview box, and content box from the GTK GUI builder. When the overview button is clicked, it replaces the content of the content box with the overview box and shows all the widgets within the overview box.
///
fn set_overview_button(builder: &Builder) -> Result<(), InterfaceError>{
    let overview_button: Button = builder.get_object(OVERVIEW_BUTTON).ok_or(InterfaceError::MissingButton)?;
    let overview_box = builder.get_object(OVERVIEW_BOX).ok_or(InterfaceError::MissingBox)?;
    let content_box: Box = builder.get_object(CONTENT_BOX).ok_or(InterfaceError::MissingBox)?;

    overview_button.connect_clicked(move |_| {
        replace_content(&content_box,&overview_box);
        overview_box.show_all();
    });
    Ok(())
}

/// Sets the functionality of the send button.
///
/// # Arguments
///
/// * `builder` - A reference to the builder object.
///
/// # Errors
///
/// Returns an `InterfaceError` if the button, frame, or box objects are missing.
fn set_send_button(builder: &Builder) -> Result<(), InterfaceError>{
    let overview_button: Button = builder.get_object(SEND_BUTTON).ok_or(InterfaceError::MissingButton)?;
    let send_frame = builder.get_object(SEND_FRAME).ok_or(InterfaceError::MissingFrame)?;
    let transaction_box: Box = builder.get_object(TRANSACTION_BOX).ok_or(InterfaceError::MissingBox)?;
    let content_box: Box = builder.get_object(CONTENT_BOX).ok_or(InterfaceError::MissingBox)?;
    let new_transaction: Fixed = create_transaction_view(transaction_box.clone())?;
    transaction_box.add(&new_transaction);

    overview_button.connect_clicked(move |_| {
        replace_content(&content_box,&send_frame);
        send_frame.show_all();
    });
    Ok(())
}


/// Sets up the receive button in the user interface.
///
/// # Arguments
///
/// * `builder` - A reference to the GTK GUI builder.
///
/// # Returns
///
/// Returns `Result<(), InterfaceError>` indicating the success or failure of setting up the receive button. An `Ok` value is returned upon successful setup, while an `Err` value of `InterfaceError` type is returned in case of any errors.
///
/// # Description
///
/// This function sets up the receive button in the user interface. It retrieves the necessary GTK objects from the builder, such as the receive button, receive frame, content box, and copy button. When the receive button is clicked, it sets the button style for the copy button, shows the copy button, replaces the content of the content box with the receive frame, and shows all the widgets within the receive frame.
///
fn set_receive_button(builder: &Builder) -> Result<(), InterfaceError>{
    let receive_button: Button = builder.get_object(RECEIVE_BUTTON).ok_or(InterfaceError::MissingButton)?;
    let receive_frame= builder.get_object(RECEIVE_FRAME).ok_or(InterfaceError::MissingFrame)?;
    let content_box: Box = builder.get_object(CONTENT_BOX).ok_or(InterfaceError::MissingBox)?;
    let copy_button: Button = builder.get_object(COPY_BUTTON).ok_or(InterfaceError::MissingButton)?;

    receive_button.connect_clicked(move |_| {
        if let Err(err) = set_button_style(&copy_button,COPY_BUTTON_STYLE1,COPY_BUTTON_STYLE2,COPY_BUTTON_STYLE3){
            println!("{:?}", err);
        };
        copy_button.show_all();
        replace_content(&content_box, &receive_frame);
        receive_frame.show_all();
    });
    Ok(())
}

// Sets the functionality of the transactions button.
///
/// # Arguments
///
/// * `builder` - A reference to the builder object.
/// * `store` - A reference to the list store.
///
/// # Errors
///
/// Returns an `InterfaceError` if the button or box objects are missing.
fn set_transactions_button(builder: &Builder, store: &ListStore) -> Result<(), InterfaceError>{
    let transactions_button: Button = builder.get_object(TRANSACTIONS_BUTTON).ok_or(InterfaceError::MissingButton)?;
    let transactions_tree_view = create_transaction_tree_view(store).upcast();
    let content_box: Box = builder.get_object(CONTENT_BOX).ok_or(InterfaceError::MissingBox)?;

    transactions_button.connect_clicked(move |_| {
        replace_content(&content_box, &transactions_tree_view);
        transactions_tree_view.show_all();
    });
    Ok(())
}

/// Clears the text in the provided GTK entry widget.
///
/// # Arguments
///
/// * `entry` - A reference to the GTK `Entry` widget.
///
/// # Description
///
/// This function clears the text in the provided GTK `Entry` widget by setting its text content to an empty string.
///
fn clean_entry(entry:&Entry) {
    entry.set_text(EMPTY);
}

/// Checks if the provided username is valid.
///
/// # Arguments
///
/// * `username` - A string slice representing the username to be validated.
///
/// # Returns
///
/// Returns a boolean value indicating whether the username is valid (`true`) or not (`false`).
///
/// # Description
///
/// This function validates the provided username by checking its length. It returns `true` if the length of the username is greater than 0 and less than 20, indicating that it meets the required criteria for a valid username.
///
fn is_username_valid(username:&str) -> bool {
    !username.is_empty() && username.len() < 20
} 

/// Checks if the provided public key is valid.
///
/// # Arguments
///
/// * `public_key` - A string slice representing the public key to be validated.
///
/// # Returns
///
/// Returns a boolean value indicating whether the public key is valid (`true`) or not (`false`).
///
/// # Description
///
/// This function validates the provided public key by checking its length and ensuring that all characters are ASCII hexadecimal digits. It returns `true` if the length of the public key is 66 (indicating the expected length for a valid public key) and all characters in the public key are ASCII hexadecimal digits.
///
fn is_pubkey_valid(public_key: &str) -> bool {
    public_key.len() == 66 && public_key.chars().all(|c| c.is_ascii_hexdigit())
}


/// Checks if the provided private key is valid.
///
/// # Arguments
///
/// * `private_key` - A string slice representing the private key to be validated.
///
/// # Returns
///
/// Returns a boolean value indicating whether the private key is valid (`true`) or not (`false`).
///
/// # Description
///
/// This function validates the provided private key by checking its length and ensuring that all characters are ASCII hexadecimal digits. It returns `true` if the length of the private key is 64 (indicating the expected length for a valid private key) and all characters in the private key are ASCII hexadecimal digits.
///
fn is_private_key_valid(private_key:&str) -> bool {
    private_key.len() == 64 && private_key.chars().all(|c| c.is_ascii_hexdigit())
} 

fn does_pubkey_match_address(pubkey: &[u8], address: &str) -> bool{
    if !is_string_bech32(address.to_string()){
        return false;
    }
    
    let addr = String::from_utf8_lossy(&address_from_pubkey(&pubkey, true)).to_string();
    
    addr == address
}

/// Sets up the login button and its associated functionality.
///
/// # Arguments
///
/// * `builder` - A reference to the GTK `Builder` object.
/// * `accounts` - An `Arc<Mutex<Accounts>>` representing the shared account data.
///
/// # Returns
///
/// Returns a `Result` indicating success (`Ok(())`) or an `InterfaceError` if any required objects are missing.
///
/// # Description
///
/// This function sets up the login button by retrieving the necessary GTK objects from the builder, connecting the click event, and defining the login logic. Upon clicking the login button, the function performs input validation, shows an authentication error dialog if the input is invalid, clears the entry fields, hides the login window, and displays the main window with the updated account information.
fn set_login_button(builder: &Builder, accounts:Arc<Mutex<Accounts>>) -> Result<(), InterfaceError>{
    let login_button: Button = builder.get_object("login_button").ok_or(InterfaceError::MissingButton)?;
    let main_window: Window = builder.get_object("main_window").ok_or(InterfaceError::MissingWindow)?;
    let login_window: Window = builder.get_object("login_window").ok_or(InterfaceError::MissingWindow)?;
    
    let user_authentication_dialog: Dialog = builder.get_object("user_authentication_window").ok_or(InterfaceError::MissingDialog)?;
    let title_label: Label = builder.get_object("title_error_label").ok_or(InterfaceError::MissingLabel)?;
    let advice_label: Label = builder.get_object("advice_label").ok_or(InterfaceError::MissingLabel)?;

    let overview_box: Widget = builder.get_object(OVERVIEW_BOX).ok_or(InterfaceError::MissingBox)?;
    let content_box: Box = builder.get_object(CONTENT_BOX).ok_or(InterfaceError::MissingBox)?;
    content_box.add(&overview_box);

    let accounts_box: Box = builder.get_object(ACCOUNTS_BOX).ok_or(InterfaceError::MissingBox)?;
    let actual_account_label: Label = builder.get_object("actual_account_label").ok_or(InterfaceError::MissingLabel)?;

    // Create a CSS provider and load CSS data to define the color
    let css_provider = CssProvider::new();
    
    css_provider.load_from_path(STYLE_PATH)?;
    
    // Add the CSS provider to the style context
    let style_context = login_button.get_style_context();
    
    style_context.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
   
    style_context.add_class(GREEN_BUTTON);

    let username_entry:Entry = builder.get_object(USERNAME_ENTRY).ok_or(InterfaceError::MissingEntry)?;
   
    let public_key_entry:Entry = builder.get_object(PUBLIC_KEY_ENTRY).ok_or(InterfaceError::MissingEntry)?;
    
    let private_key_entry:Entry = builder.get_object(PRIVATE_KEY_ENTRY).ok_or(InterfaceError::MissingEntry)?;

    let address_entry:Entry = builder.get_object(ADDRESS_ENTRY).ok_or(InterfaceError::MissingEntry)?;
    
    let public_key_to_copy:Label = builder.get_object(SHARED_PUBKEY).ok_or(InterfaceError::MissingLabel)?;
    
    login_button.connect_clicked(move |_| {
        let username = username_entry.get_text();
        let public_key = public_key_entry.get_text();
        let private_key = private_key_entry.get_text();
        let address = address_entry.get_text();

        if !is_username_valid(username.as_str()) || !is_pubkey_valid(public_key.as_str()) || !is_private_key_valid(private_key.as_str()) {
            let mut auth_text = "Please, fill in the inputs correctly".to_string();

            if !is_username_valid(username.as_str()) {
                auth_text += "\n \n - Invalid username";
            }
            
            if !is_pubkey_valid(public_key.as_str()) {
                auth_text += "\n \n - Invalid public key";
            }

            if !is_private_key_valid(private_key.as_str()) {
                auth_text += "\n \n - Invalid private key";
            }

            title_label.set_text("Login Authentication Error");
            advice_label.set_text(auth_text.as_str());
            user_authentication_dialog.show_all();
            return
        }

        let new_account_button = Button::new();
        new_account_button.set_label(&username);

        let username_account = username.to_string();
        let shared_accounts = accounts.clone();
        let shared_actual_account_label = actual_account_label.clone();
        new_account_button.connect_clicked(move |_| {
            if let Ok(mut locked_accounts) = shared_accounts.lock(){
                locked_accounts.set_actual_account(username_account.clone());
                shared_actual_account_label.set_text(&username_account.clone());
            }
        });

        clean_entry(&username_entry);
        clean_entry(&public_key_entry);
        clean_entry(&private_key_entry);
        clean_entry(&address_entry);

        login_window.hide();

        //wallet_combo_box.append_text(&username);
    
        public_key_to_copy.set_text(&public_key);
    
        if let Ok(mut accounts) = accounts.lock() {
            if let Ok(public_key_bytes) = decode_hex(&public_key){
                if let Ok(private_key_bytes) = decode_hex(&private_key){
                    actual_account_label.set_text(&username);
                    accounts_box.add(&new_account_button);

                    let bech32 = does_pubkey_match_address(&public_key_bytes, &address);

                    accounts.add_account(username.to_string(), public_key_bytes, private_key_bytes, bech32);
                    main_window.show_all();
                }
            }
            drop(accounts);
        }
    });

    Ok(())
}

/// Sets up the add recipient button and its associated functionality.
///
/// # Arguments
///
/// * `builder` - A reference to the GTK `Builder` object.
///
/// # Returns
///
/// Returns a `Result` indicating success (`Ok(())`) or an `InterfaceError` if any required objects are missing.
///
/// # Description
///
/// This function sets up the add recipient button by retrieving the necessary GTK objects from the builder, connecting the click event, and defining the logic to add a new recipient. Upon clicking the add recipient button, the function creates a new transaction view and adds it to the transaction box. The new transaction view is then shown.
///
fn set_add_recipient_button(builder: &Builder) -> Result<(), InterfaceError>{
    let add_recipient_button: Button = builder.get_object(ADD_RECIPIENT_BUTTON).ok_or(InterfaceError::MissingButton)?;
    let transaction_box: Box = builder.get_object(TRANSACTION_BOX).ok_or(InterfaceError::MissingBox)?;

    add_recipient_button.connect_clicked(move |_| {
        //let transaction_count = transaction_box.get_children().len();

        if let Ok(new_transaction) = create_transaction_view(transaction_box.clone()){
            transaction_box.add(&new_transaction);
            new_transaction.show_all();
        };
    });
    Ok(())
}

/// Clears the contents of a GTK box and adds a new widget to it.
///
/// # Arguments
///
/// * `gtk_box` - A reference to the GTK `Box` object.
/// * `widget` - A reference to the GTK `Widget` to be added.
///
/// # Description
///
/// This function removes all existing children from the provided GTK box and adds a new widget to it. The widget is then shown.
///
fn clear_and_add_widget(gtk_box:&Box,widget:&Widget) {
    gtk_box.foreach(|child| {
        gtk_box.remove(child);
    });
    gtk_box.add(widget);
    widget.show_all();
}

/// Sets up the functionality for the "Clear All" button.
///
///
/// # Arguments
///
/// * `builder` - A reference to the GTK builder.
///
/// # Returns
///
/// * `Result<(), InterfaceError>` - A result indicating success or failure. Returns `Ok(())`
///   if the button setup was successful, or an `InterfaceError` if any required objects are missing.
///
///  # Description
/// 
/// The "Clear All" button, when clicked, clears all transaction-related widgets from the
/// transaction box and adds a new transaction view widget.
/// 
fn set_clear_all_button(builder: &Builder) -> Result<(), InterfaceError>{
    let clear_all_button: Button = builder.get_object(CLEAR_ALL_BUTTON).ok_or(InterfaceError::MissingLabel)?;
    let transaction_box: Box = builder.get_object(TRANSACTION_BOX).ok_or(InterfaceError::MissingBox)?;

    clear_all_button.connect_clicked(move |_| {
        if let Ok(new_transaction) = create_transaction_view(transaction_box.clone()){
            clear_and_add_widget(&transaction_box, new_transaction.upcast_ref());
        }
    });
    Ok(())
}


/// The copy button allows the user to copy the public key text to the clipboard. 
/// When clicked, the public key text is copied to the clipboard, and the button style is updated to indicate a successful copy.
///
/// # Arguments
///
/// * `builder` - A reference to the GTK builder used to access UI elements.
///
/// # Returns
///
/// Returns `Ok(())` if the copy button setup is successful, or an `InterfaceError` if any required UI element is missing.
///
/// # Description
///
/// This function sets up the copy button by connecting its `clicked` signal to a closure that performs the copy functionality. 
/// It retrieves the copy button and the label containing the public key text from the builder. When the copy button is clicked, 
/// the closure is executed. It gets the default clipboard and sets the public key text as the clipboard content. Then, it calls 
/// the `set_button_style` function to update the button style to indicate a successful copy. If any error occurs during this 
/// process, it prints the error message to the console.
fn set_copy_button(builder:&Builder) -> Result<(), InterfaceError>{
    let copy_button: Button = builder.get_object(COPY_BUTTON).ok_or(InterfaceError::MissingButton)?;
    let public_key_to_copy:Label = builder.get_object(SHARED_PUBKEY).ok_or(InterfaceError::MissingLabel)?;

    copy_button.connect_clicked(move |copy_button| {
        if let Some(clipboard) = Clipboard::get_default(&copy_button.get_display()) {
            clipboard.set_text(public_key_to_copy.get_text().as_str()); 
        }
        if let Err(err) = set_button_style(copy_button, COPY_BUTTON_STYLE2,COPY_BUTTON_STYLE1, COPY_BUTTON_STYLE4) {
            println!("{:?}", err);
        };
    });

    Ok(())
}


/// Sets up the functionality for the Point of Interest (POI) button.
///
/// The POI button allows the user to navigate to the Point of Interest section of the application.
/// When clicked, the content box is replaced with the POI box, which displays the relevant information.
///
/// # Arguments
///
/// * `builder` - A reference to the GTK builder used to access UI elements.
///
/// # Returns
///
/// Returns `Ok(())` if the POI button setup is successful, or an `InterfaceError` if any required UI element is missing.
///
/// # Description
///
/// This function sets up the POI button by connecting its `clicked` signal to a closure that performs the navigation functionality.
/// It retrieves the POI button, POI box, and content box from the builder. When the POI button is clicked, the closure is executed.
/// It calls the `replace_content` function to replace the content in the content box with the POI box. Finally, it shows the POI box.
///
fn set_poi_button(builder: &Builder) -> Result<(), InterfaceError>{
    let poi_button: Button = builder.get_object(POI_BUTTON).ok_or(InterfaceError::MissingButton)?;
    let poi_box= builder.get_object(POI_BOX).ok_or(InterfaceError::MissingBox)?;
    let content_box: Box = builder.get_object(CONTENT_BOX).ok_or(InterfaceError::MissingBox)?;

    poi_button.connect_clicked(move |_| {
        replace_content(&content_box, &poi_box);
        poi_box.show_all();
    });
    Ok(())
}

/// Validates a block header.
///
/// Checks if the given block header is a valid hexadecimal string with a length of 64 characters.
///
/// # Arguments
///
/// * `block_header` - The block header to validate as a hexadecimal string.
///
/// # Returns
///
/// Returns `true` if the block header is valid, or `false` otherwise.
///
/// # Description
///
/// This function validates a block header by checking if it meets the following criteria:
///
/// * The block header has a length of 64 characters.
/// * All characters in the block header are valid hexadecimal digits.
///
fn valid_block_header(block_header:&str)-> bool {
    block_header.len() == 64 && block_header.chars().all(|c| c.is_ascii_hexdigit())
}

/// Validates a transaction ID.
///
/// Checks if the given transaction ID is a valid hexadecimal string with a length of 64 characters.
///
/// # Arguments
///
/// * `transaction_id` - The transaction ID to validate as a hexadecimal string.
///
/// # Returns
///
/// Returns `true` if the transaction ID is valid, or `false` otherwise.
///
/// # Description
///
/// This function validates a transaction ID by checking if it meets the following criteria:
///
/// * The transaction ID has a length of 64 characters.
/// * All characters in the transaction ID are valid hexadecimal digits.
///
fn valid_transaction_id(transaction_id:&str)-> bool {
    transaction_id.len() == 64 && transaction_id.chars().all(|c| c.is_ascii_hexdigit())
}

/// Sets up the "Make Proof" button functionality.
///
/// This function connects the "Make Proof" button to the corresponding action when clicked.
/// It retrieves the necessary UI elements from the builder and handles user input validation.
/// It then sends a request to the specified node to obtain a proof of inclusion for a given block header and transaction ID.
/// Finally, it displays the result in a dialog window.
///
/// # Arguments
///
/// * `builder` - The builder object containing the UI definition.
/// * `node` - The TCP stream representing the connection to the node.
///
/// # Returns
///
/// Returns `Ok(())` if the setup is successful, or an `InterfaceError` if any UI element is missing.
///
/// # Description
///
/// This function sets up the "Make Proof" button by performing the following steps:
///
/// 1. Retrieves the "Make Proof" button, block header entry, and transaction ID entry from the builder.
/// 2. Retrieves the necessary dialog elements for error display.
/// 3. Connects the "Make Proof" button's click event to the corresponding action.
/// 4. Validates the block header and transaction ID input provided by the user.
/// 5. Sends a request to the specified node to obtain a proof of inclusion.
/// 6. Displays the result in a dialog window.
///
fn set_make_proof_button(builder: &Builder, node: Arc<Mutex<TcpStream>>) -> Result<(), InterfaceError> {
    let make_proof_button: Button = builder.get_object(MAKE_PROOF_BUTTON).ok_or(InterfaceError::MissingButton)?;
    let block_header_entry: Entry = builder.get_object(BLOCK_HEADER_ENTRY).ok_or(InterfaceError::MissingEntry)?;
    let transaction_id_entry: Entry = builder.get_object(TRANSACTION_ID_ENTRY).ok_or(InterfaceError::MissingEntry)?;

    let user_authentication_dialog: Dialog = builder.get_object("user_authentication_window").ok_or(InterfaceError::MissingDialog)?;
    let title_label: Label = builder.get_object("title_error_label").ok_or(InterfaceError::MissingLabel)?;
    let advice_label: Label = builder.get_object("advice_label").ok_or(InterfaceError::MissingLabel)?;

    let poi_dialog: Dialog = builder.get_object("proof_of_inclusion_window").ok_or(InterfaceError::MissingDialog)?;
    let poi_title_label: Label = builder.get_object("poi_status_title").ok_or(InterfaceError::MissingLabel)?;
    let poi_advice_label: Label = builder.get_object("advice_poi_label").ok_or(InterfaceError::MissingLabel)?;
    let poi_error_image: Image = builder.get_object("poi_error_image").ok_or(InterfaceError::MissingImage)?;
    let poi_success_image: Image = builder.get_object("poi_success_image").ok_or(InterfaceError::MissingImage)?;

    make_proof_button.connect_clicked(move |_| {
        println!("Entra al click");

        /* let block_header = match decode_hex(&block_header_entry.get_text()) {
            Ok(block_header) => block_header,
            Err(_) => return
        };

        let tx_id: Vec<u8> = match decode_hex(&transaction_id_entry.get_text()){
            Ok(tx_id) => tx_id,
            Err(_) => return
        };

        let get_proof = GetProof::new(block_header, tx_id); */
        //let block_header: Vec<u8> = decode_hex(&block_header_entry.get_text()).unwrap();
        
        let block_header_text = block_header_entry.get_text();
        let transaction_id_text = transaction_id_entry.get_text();

        if !valid_block_header(block_header_text.as_str()) || !valid_transaction_id(transaction_id_text.as_str()) {
            let mut auth_text = "Please complete the entries correctly".to_string();
            
            if !valid_block_header(block_header_text.as_str()) {
                auth_text += "\n \n - Block header is invalid";
            }

            if !valid_transaction_id(transaction_id_text.as_str()) {
                auth_text += "\n \n - Transaction ID is invalid";
            }

            title_label.set_text("Proof of Inclusion Authentication Error");
            advice_label.set_text(auth_text.as_str());
            user_authentication_dialog.set_size_request(600, 200);
            user_authentication_dialog.show_all();

            return;
        }
        let block_header: Vec<u8> = match decode_hex(&block_header_entry.get_text()) {
            Ok(header) => header,
            Err(_) => return
        };


        let tx_id: Vec<u8> = match decode_hex(&transaction_id_entry.get_text()) {
            Ok(tx_id) => tx_id,
            Err(_) => return
        };
        let get_proof = GetProof::new(block_header, tx_id);
        
        let mut locked_node = match node.lock(){
            Ok(locked_node) => locked_node,
            Err(_) => return
        };

        println!("Se lockea bien el nodo");

        if locked_node.write_all(&get_proof.as_bytes()).is_err(){
            return
        };
        
        println!("Se envia el get_proof: {:?}", get_proof);

        let command_name = match read_string_from_bytes(&mut *locked_node, 12){
            Ok(command_name) => command_name,
            Err(_) => return
        };

        println!("Command name despues de get proof: {}", command_name);
        if command_name != MERKLE_BLOCK { //implementar mensaje
            poi_title_label.set_text("Invalid Proof of Inclusion");
            poi_advice_label.set_text("The transaction was not found in block");
            poi_success_image.hide();
            poi_dialog.show();
            println!("Entra al return");
            drop(locked_node);
            return 
        } 
        println!("\npasa\n");
        let merkle_block = match MerkleBlock::from_bytes(command_name, &mut *locked_node){
            Ok(merkle_block) => merkle_block,
            Err(_) => return
        };

        /* println!("después del merkle block\n");
        if let Ok(proof_of_inclusion) = check_proof_of_inclusion(merkle_block){
            println!("{}", proof_of_inclusion);
        }; */
        println!("despues del merkle block\n");
        let proof_of_inclusion = match get_proof_of_inclusion(merkle_block) {
            Ok(proof) => proof,
            Err(_) => return
        };

        println!("{}", proof_of_inclusion);

        if proof_of_inclusion {
            poi_title_label.set_text("Successful Proof of Inclusion");
            poi_advice_label.set_text("The transaction was successfully verified in block");
            poi_error_image.hide();
        }
        else {
            poi_title_label.set_text("Invalid Proof of Inclusion");
            poi_advice_label.set_text("The transaction was not found in block");
            poi_success_image.hide();
        }

        poi_dialog.show();

        drop(locked_node);
    });

    Ok(())
}


/// Retrieves the target list from the transaction box.
///
/// This function iterates over the children of the transaction box and extracts the target values
/// (byte arrays) and amounts (as integers) from the corresponding UI elements.
///
/// # Arguments
///
/// * `transaction_box` - The Box object representing the transaction box container.
///
/// # Returns
///
/// Returns a vector of tuples containing the target values and amounts.
///
/// # Description
///
/// This function retrieves the target list from the transaction box by performing the following steps:
///
/// 1. Initializes an empty vector to store the target values and amounts.
/// 2. Iterates over each transaction UI element within the transaction box.
/// 3. For each transaction, extracts the target value and amount by iterating over its child elements.
/// 4. Checks if the child element is a SpinButton or Entry widget and retrieves the corresponding value.
/// 5. Stores the target value and amount as a tuple in the vector.
/// 6. Returns the vector of target values and amounts.
///
fn get_target_list(transaction_box:&Box) -> Vec<(Vec<u8>,i64)>{
    let mut target_list = vec![];

    for tx in transaction_box.get_children() {
        let mut target:(Vec<u8>,i64) = (vec![],0);
        if let Some(tx_fixed) = tx.downcast_ref::<Fixed>() {
            for fixed_child in tx_fixed.get_children() {
                if let Some(spin_button) = fixed_child.downcast_ref::<gtk::SpinButton>() {
                    target.1 = (spin_button.get_value()*100000000.0) as i64;
                } else if let Some(entry) = fixed_child.downcast_ref::<gtk::Entry>() {
                    target.0 = entry.get_text().as_bytes().to_vec();
                }
            }
        } 
        target_list.push(target);
    }

    target_list
}

/// Sets up the functionality for the "Send Transaction" button.
///
/// This function connects the "Send Transaction" button to a click event handler. When clicked, it
/// retrieves the target list, fee value, and private key from the UI elements and attempts to create
/// a transaction. If the transaction creation is successful, it creates a new transaction view and
/// updates the transaction box accordingly.
///
/// # Arguments
///
/// * `builder` - The Builder object for accessing UI elements.
/// * `node` - An Arc-wrapped Mutex-wrapped TcpStream for communication with the node.
/// * `accounts` - An Arc-wrapped Mutex for accessing account information.
///
/// # Returns
///
/// Returns `Ok(())` if the function executes successfully, or an `InterfaceError` if any UI elements are missing.
///
/// # Description
///
/// This function sets up the functionality for the "Send Transaction" button by performing the following steps:
///
/// 1. Retrieves the necessary UI elements from the builder, including the "Send Transaction" button,
///    transaction box, fee spin button, and account information.
/// 2. Connects the "Send Transaction" button to a click event handler using the `connect_clicked` method.
/// 3. In the click event handler, retrieves the target list, fee value, and private key from the UI elements.
/// 4. Acquires a lock on the accounts Mutex to access the account information.
/// 5. If the account information is available, attempts to create a transaction using the `create_transaction` function.
/// 6. If the transaction creation is successful, creates a new transaction view and updates the transaction box.
/// 7. Handles any errors that may occur during the transaction creation process, such as insufficient funds.
/// 8. Drops the lock on the accounts Mutex.
/// 9. Returns `Ok(())` if the function executes successfully.
///
fn set_send_transaction_button(builder: &Builder, node: Arc<Mutex<TcpStream>>, accounts: Arc<Mutex<Accounts>>) -> Result<(), InterfaceError> {
    let send_transaction_button: Button = builder.get_object(SEND_TX_BUTTON).ok_or(InterfaceError::MissingButton)?;
    let transaction_box: Box = builder.get_object(TX_BOX).ok_or(InterfaceError::MissingBox)?;
    let spin_button_fee: SpinButton = builder.get_object(FEE_SPIN_BUTTON).ok_or(InterfaceError::MissingSpinButton)?;

    send_transaction_button.connect_clicked( move |_| {
        println!("\n\n\nHOLAAAAAAA\n\n\n");
        let target_list = get_target_list(&transaction_box);

        if let Ok(locked_accounts) = accounts.lock(){
            if let Some(user_info) = locked_accounts.get_current_account_info(){
                let private_key = user_info.get_private_key();
                let fee = spin_button_fee.get_value() * 100000000.0;

                match create_transaction(target_list, user_info.get_utxo(), private_key, fee as i64, user_info.get_bech32()){
                    Ok(transaction) => {
                        if let Ok(mut locked_node) = node.lock() {
                            println!("Connection established to broadcast transaction:\n{:?}", transaction);
                            let broadcast_txn = BroadcastTxn::new(transaction.clone());
                            println!("Message: {:?}", broadcast_txn);

                            println!("\n\n\n{:?}\n\n\n", encode_hex(&transaction.as_bytes(true)).unwrap());

                            if locked_node.write_all(&broadcast_txn.as_bytes(user_info.get_bech32())).is_err(){
                                println!("Error when broadcasting new transaction to node.");
                                return;
                            }

                            if let Ok(new_transaction) = create_transaction_view(transaction_box.clone()){
                                clear_and_add_widget(&transaction_box, new_transaction.upcast_ref());
                            }
                            drop(locked_node);
                        /* if let Ok(new_transaction) = create_transaction_view(transaction_box.clone()) {
                            clear_and_add_widget(&transaction_box, &new_transaction.upcast_ref());
                            println!("{:?}", encode_hex(&transaction.as_bytes()).unwrap());
                        } */
                    }},
                    Err(TransactionCreateError::InsufficientFounds) => {
                        println!("Insufficient balance in account.");
                    },
                    _ => {}
                }
            }
            drop(locked_accounts);
        }
    });

    Ok(())
}


/// Sets up the functionality for the "Send Transaction" button.
///
/// This function connects the "Send Transaction" button to a click event handler. When clicked, it
/// retrieves the target list, fee value, and private key from the UI elements and attempts to create
/// a transaction. If the transaction creation is successful, it creates a new transaction view and
/// updates the transaction box accordingly.
///
/// # Arguments
///
/// * `builder` - The Builder object for accessing UI elements.
/// * `node` - An Arc-wrapped Mutex-wrapped TcpStream for communication with the node.
/// * `accounts` - An Arc-wrapped Mutex for accessing account information.
///
/// # Returns
///
/// Returns `Ok(())` if the function executes successfully, or an `InterfaceError` if any UI elements are missing.
///
/// # Description
///
/// This function sets up the functionality for the "Send Transaction" button by performing the following steps:
///
/// 1. Retrieves the necessary UI elements from the builder, including the "Send Transaction" button,
///    transaction box, fee spin button, and account information.
/// 2. Connects the "Send Transaction" button to a click event handler using the `connect_clicked` method.
/// 3. In the click event handler, retrieves the target list, fee value, and private key from the UI elements.
/// 4. Acquires a lock on the accounts Mutex to access the account information.
/// 5. If the account information is available, attempts to create a transaction using the `create_transaction` function.
/// 6. If the transaction creation is successful, creates a new transaction view and updates the transaction box.
/// 7. Handles any errors that may occur during the transaction creation process, such as insufficient funds.
/// 8. Drops the lock on the accounts Mutex.
/// 9. Returns `Ok(())` if the function executes successfully.
///
fn set_new_account_button(builder: &Builder)-> Result<(), InterfaceError> {
    let main_window: Window = builder.get_object(MAIN_WINDOW).ok_or(InterfaceError::MissingWindow)?;
    let login_window: Window = builder.get_object(LOGIN_WINDOW).ok_or(InterfaceError::MissingWindow)?;
    let new_account_button: Button = builder.get_object(NEW_ACCOUNT_BUTTON).ok_or(InterfaceError::MissingWindow)?;
    let return_button: Button = builder.get_object(RETURN_BUTTON).ok_or(InterfaceError::MissingWindow)?;

    new_account_button.connect_clicked(move |_| {
        main_window.hide();
        return_button.show();
        login_window.show_all();
    });

    Ok(())
}

/// Sets up the functionality for the "Return" button.
///
/// This function connects the "Return" button to a click event handler. When clicked, it hides the login window,
/// cleans the username, public key, and private key entries, and shows the main window.
///
/// # Arguments
///
/// * `builder` - The Builder object for accessing UI elements.
///
/// # Returns
///
/// Returns `Ok(())` if the function executes successfully, or an `InterfaceError` if any UI elements are missing.
///
/// # Description
///
/// This function sets up the functionality for the "Return" button by performing the following steps:
///
/// 1. Retrieves the necessary UI elements from the builder, including the main window, login window,
///    "Return" button, and username, public key, and private key entries.
/// 2. Connects the "Return" button to a click event handler using the `connect_clicked` method.
/// 3. In the click event handler, hides the login window, cleans the username, public key, and private key entries,
///    and shows the main window.
/// 4. Returns `Ok(())` if the function executes successfully.
///
fn set_return_button(builder:&Builder)-> Result<(), InterfaceError> {
    let main_window: Window = builder.get_object(MAIN_WINDOW).ok_or(InterfaceError::MissingWindow)?;
    let login_window: Window = builder.get_object(LOGIN_WINDOW).ok_or(InterfaceError::MissingWindow)?;
    let return_button: Button = builder.get_object(RETURN_BUTTON).ok_or(InterfaceError::MissingWindow)?;

    let username_entry:Entry = builder.get_object(USERNAME_ENTRY).ok_or(InterfaceError::MissingEntry)?;
    let public_key_entry:Entry = builder.get_object(PUBLIC_KEY_ENTRY).ok_or(InterfaceError::MissingEntry)?;
    let private_key_entry:Entry = builder.get_object(PRIVATE_KEY_ENTRY).ok_or(InterfaceError::MissingEntry)?;

    return_button.connect_clicked(move |_| {
        login_window.hide();
        clean_entry(&username_entry);
        clean_entry(&public_key_entry);
        clean_entry(&private_key_entry);
        main_window.show_all();
    });

    Ok(())
}

/// Sets up the functionality for the "OK" buttons in the user authentication and proof of inclusion dialogs.
///
/// This function connects the "OK" buttons to click event handlers. When clicked, it hides the respective dialog.
///
/// # Arguments
///
/// * `builder` - The Builder object for accessing UI elements.
///
/// # Returns
///
/// Returns `Ok(())` if the function executes successfully, or an `InterfaceError` if any UI elements are missing.
///
/// # Description
///
/// This function sets up the functionality for the "OK" buttons in the user authentication and proof of inclusion dialogs
/// by performing the following steps:
///
/// 1. Retrieves the necessary UI elements from the builder, including the user authentication dialog, proof of inclusion dialog,
///    "OK" button in the user authentication dialog, and "OK" button in the proof of inclusion dialog.
/// 2. Connects the "OK" buttons to click event handlers using the `connect_clicked` method.
/// 3. In the click event handlers, hides the respective dialog.
/// 4. Returns `Ok(())` if the function executes successfully.
///
fn set_ok_button(builder:&Builder) -> Result<(), InterfaceError>{
    let user_authentication_dialog: Dialog = builder.get_object("user_authentication_window").ok_or(InterfaceError::MissingDialog)?;
    let poi_dialog: Dialog = builder.get_object("proof_of_inclusion_window").ok_or(InterfaceError::MissingDialog)?;
    let ok_button: Button = builder.get_object("ok_button").ok_or(InterfaceError::MissingButton)?;
    let poi_ok_button: Button = builder.get_object("poi_ok_button").ok_or(InterfaceError::MissingButton)?;

    ok_button.connect_clicked(move |_| {
        user_authentication_dialog.hide();
    });

    poi_ok_button.connect_clicked(move |_| {
        poi_dialog.hide();
    });

    Ok(())
}
/* 
fn set_combo_box_button(builder: &Builder, accounts: Arc<Mutex<Accounts>>) -> Result<(), InterfaceError> {
    let accounts_box: Box = builder.get_object(ACCOUNTS_BOX).ok_or(InterfaceError::MissingBox)?;

    
    if let Ok(mut locked_accounts) = accounts.lock(){
        locked_accounts.set_actual_account(active_text.to_string());
        drop(locked_accounts);
    };
        
    Ok(())

} */
// cuando se hace login agregar un button y setearlo
// si se clickea el boton, agarrar el texto y setear el locked accounts con eso

