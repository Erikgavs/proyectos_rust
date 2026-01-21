
# 🪓Freya Projects

In this folder will be Rust-Projects that uses freya (UI Rust framework).

Here will be a lot's of differents aplications maded with freya


# 📖 Projects

## 🌍 Hello World

A foundational project designed as an introduction to the Freya UI framework. This application displays a simple centered greeting message ("Hello Word! By Erik!") demonstrating the basic structure of a Freya application including layout containers, text rendering, and styling properties.

**Key Concepts Demonstrated:**
- Basic Freya app setup with `launch()` and `rsx!` macro
- Layout management using `rect` containers
- Text rendering with `label` elements
- Center alignment and font sizing

[Link to the project](https://github.com/Erikgavs/proyectos_rust/tree/main/freya_projects/hello_world)

---

## 🪟 Windows

An interactive application showcasing modal popup functionality in Freya. The app features a button that, when pressed, triggers a popup overlay. This project demonstrates state management and event handling for creating interactive UI elements.

**Key Concepts Demonstrated:**
- Reactive state management with `use_signal`
- Conditional rendering based on state
- Event handling with button press actions
- Modal popup component (`Popup`) implementation
- Close request handling

[Link to the project](https://github.com/Erikgavs/proyectos_rust/tree/main/freya_projects/windows)

---

## 📝 New Note

A practical application that combines popup windows with user input functionality. The app features a button that opens a modal dialog containing an input field for creating new notes. This project demonstrates the integration of multiple UI concepts including modals, forms, and state management.

**Key Concepts Demonstrated:**
- Combined use of popups and input fields
- Multi-state management (popup visibility + input value)
- Form-like interaction patterns
- User input capture and handling

[Link to the project](https://github.com/Erikgavs/proyectos_rust/tree/main/freya_projects/new_note)

---

## 🙊 Your Name

An application demonstrating form input handling and real-time data binding in Freya. Users can enter their name in an input field, which is displayed in real-time above the input. A submit button processes the input, logs it to the console, and clears the field, showcasing a complete input-output workflow.

**Key Concepts Demonstrated:**
- Text input component (`Input`) implementation
- Real-time reactive value binding
- Event handling with `onchange` and `onpress`
- State mutation and reset
- Console logging integration

[Link to the project](https://github.com/Erikgavs/proyectos_rust/tree/main/freya_projects/yourname)
