import Counter
import SwiftUI

struct AppNavigation: View {
    @Bindable var rust: ViewModel
    @State private var showErrorScreen = false
    @State private var showSuccessScreen = false
    @State private var successMessage = "Transaction completed successfully!"
    @State private var errorMessage = "Transaction failed. Please try again."

    // Public initializer
    init(rust: ViewModel) {
        self.rust = rust
        print("AppNavigation initialized with ViewModel ID: \(ObjectIdentifier(rust))")
    }

    var body: some View {
        NavigationStack(path: $rust.router.routes) {
            // Main home view
            HomeView(rust: rust)
                // Define navigation destinations for each route type
                .navigationDestination(for: Route.self) { route in
                    switch route {
                    case .mint:
                        MintView(rust: rust)
                            .navigationBarBackButtonHidden(true)
                            .toolbar {
                                ToolbarItem(placement: .navigationBarLeading) {
                                    Button {
                                        rust.dispatch(event: .popRoute)
                                    } label: {
                                        Image(systemName: "chevron.left")
                                            .imageScale(.large)
                                    }
                                }
                            }
                    case .mintAmount:
                        MintAmountView(rust: rust)
                            .navigationBarBackButtonHidden(true)
                            .toolbar {
                                ToolbarItem(placement: .navigationBarLeading) {
                                    Button {
                                        rust.dispatch(event: .popRoute)
                                    } label: {
                                        Image(systemName: "chevron.left")
                                            .imageScale(.large)
                                    }
                                }
                            }
                    case .mintConfirm:
                        MintConfirmView(rust: rust)
                            .navigationBarBackButtonHidden(true)
                            .toolbar {
                                ToolbarItem(placement: .navigationBarLeading) {
                                    Button {
                                        rust.dispatch(event: .popRoute)
                                    } label: {
                                        Image(systemName: "chevron.left")
                                            .imageScale(.large)
                                    }
                                }
                            }
                    case .melt:
                        MeltView(rust: rust)
                            .navigationBarBackButtonHidden(true)
                            .toolbar {
                                ToolbarItem(placement: .navigationBarLeading) {
                                    Button {
                                        rust.dispatch(event: .popRoute)
                                    } label: {
                                        Image(systemName: "chevron.left")
                                            .imageScale(.large)
                                    }
                                }
                            }
                    case .meltConfirm:
                        MeltConfirmView(rust: rust)
                            .navigationBarBackButtonHidden(true)
                            .toolbar {
                                ToolbarItem(placement: .navigationBarLeading) {
                                    Button {
                                        rust.dispatch(event: .popRoute)
                                    } label: {
                                        Image(systemName: "chevron.left")
                                            .imageScale(.large)
                                    }
                                }
                            }
                    default:
                        EmptyView()
                    }
                }
        }
        .onChange(of: rust.currentRoute) { _, newRoute in
            // Unwrap the optional route before using it
            if let newRoute = newRoute {
                // Handle special routes
                if newRoute == .success {
                    // Show success screen
                    showSuccessScreen = true
                } else if newRoute == .error {
                    // Show error screen
                    showErrorScreen = true
                }
            }
        }
        .fullScreenCover(isPresented: $showErrorScreen) {
            ErrorView(
                rust: rust,
                error: errorMessage,
                onRetry: {
                    showErrorScreen = false
                },
                onQuit: {
                    rust.dispatch(event: .resetRouter)
                    showErrorScreen = false
                }
            )
        }
        .fullScreenCover(isPresented: $showSuccessScreen) {
            SuccessView(rust: rust, message: successMessage) {
                // Reset to home on dismiss
                rust.dispatch(event: .resetRouter)
                showSuccessScreen = false
            }
        }
    }
}

// MARK: - View Implementations

struct HomeView: View {
    @Bindable var rust: ViewModel
    @State private var postText: String = "hey this is a post under 140 characters!"
    @State private var characterCount: Int = 100
    
    init(rust: ViewModel) {
        self.rust = rust
        print("HomeView initialized with ViewModel ID: \(ObjectIdentifier(rust))")
    }
    
    var body: some View {
        VStack(spacing: 0) {
            // Title
            Text("Poast")
                .font(.largeTitle)
                .fontWeight(.bold)
                .foregroundColor(.primary)
                .padding()
            
            if let eventId = rust.postEventId {
                // Success state
                VStack(spacing: 20) {
                    Text("Post sent!")
                        .font(.title)
                        .foregroundColor(.green)
                    
                    Text("Event ID:")
                        .font(.headline)
                    
                    Text(eventId)
                        .font(.system(.body, design: .monospaced))
                        .padding()
                        .background(
                            RoundedRectangle(cornerRadius: 8)
                                .fill(Color(.systemGray6))
                        )
                        .onTapGesture {
                            UIPasteboard.general.string = eventId
                        }
                        .overlay(
                            RoundedRectangle(cornerRadius: 8)
                                .stroke(Color.primary.opacity(0.2), lineWidth: 1)
                        )
                    
                    Text("(Tap to copy)")
                        .font(.caption)
                        .foregroundColor(.secondary)
                    
                    Button(action: {
                        // Reset state for new post
                        rust.postEventId = nil
                        rust.postSuccessMessage = nil
                        postText = ""
                    }) {
                        Text("New Post")
                            .fontWeight(.bold)
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color.black)
                            .foregroundColor(.white)
                            .cornerRadius(5)
                    }
                    .padding(.top, 30)
                }
                .padding()
            } else {
                // Post creation state
                // Post input area
                ZStack(alignment: .bottomTrailing) {
                    TextEditor(text: $postText)
                        .scrollContentBackground(.hidden)
                        .padding(.horizontal, 12)
                        .padding(.vertical, 8)
                        .frame(maxWidth: .infinity)
                        .frame(height: 200)
                        .background(
                            RoundedRectangle(cornerRadius: 10)
                                .stroke(Color.primary.opacity(0.3), lineWidth: 1)
                                .background(Color(.systemBackground).opacity(0.3))
                                .shadow(color: Color.primary.opacity(0.1), radius: 3, x: 0, y: 1)
                        )
                        .padding()
                    
                    // Character counter
                    ZStack {
                        Circle()
                            .stroke(Color.primary.opacity(0.3), lineWidth: 1)
                            .background(Circle().fill(Color(.systemBackground)))
                            .frame(width: 40, height: 40)
                        
                        Text("\(characterCount)")
                            .font(.system(size: 14))
                            .foregroundColor(.primary)
                    }
                    .padding(.trailing, 30)
                    .padding(.bottom, 30)
                }
                
                // Media button
                HStack {
                    Button(action: {
                        // Media button action
                    }) {
                        HStack {
                            Image(systemName: "photo")
                                .imageScale(.large)
                            Text("Media")
                                .fontWeight(.medium)
                        }
                        .padding()
                        .frame(width: 150)
                        .background(
                            RoundedRectangle(cornerRadius: 10)
                                .stroke(Color.primary.opacity(0.3), lineWidth: 1)
                        )
                    }
                    .foregroundColor(.primary)
                    
                    Spacer()
                }
                .padding(.horizontal)
                .padding(.top, 5)
                
                Spacer()
                
                // POAST button
                Button(action: {
                    rust.dispatch(event: .sendPost(text: postText))
                }) {
                    Text("POAST")
                        .fontWeight(.bold)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.black)
                        .foregroundColor(.white)
                        .cornerRadius(5)
                }
                .padding()
            }
        }
        .navigationTitle("")
        .navigationBarHidden(true)
    }
}

struct TransactionHistoryView: View {
    @Bindable var rust: ViewModel

    init(rust: ViewModel) {
        self.rust = rust
    }

    var body: some View {
        List {
            ForEach(0..<3) { _ in
                Text("21,000 sats")
                    .padding()
            }
        }
        .navigationTitle("Transaction History")
    }
}

struct MintView: View {
    @Bindable var rust: ViewModel

    init(rust: ViewModel) {
        self.rust = rust
    }

    var body: some View {
        VStack {
            Text("Choose Mint Option")
                .font(.title)

            LazyVGrid(columns: [GridItem(.flexible()), GridItem(.flexible())], spacing: 20) {
                ForEach(["A", "B", "C", "D"], id: \.self) { option in
                    Button(action: {
                        rust.dispatch(event: .pushRoute(route: .mintAmount))
                    }) {
                        Text("Mint \(option)")
                            .frame(width: 120, height: 80)
                            .background(Color.blue.opacity(0.7))
                            .foregroundColor(.white)
                            .cornerRadius(10)
                    }
                }
            }
            .padding()

            Spacer()
        }
        .navigationTitle("Mint")
    }
}

struct MintAmountView: View {
    @Bindable var rust: ViewModel
    @State private var amount: String = "12"

    init(rust: ViewModel) {
        self.rust = rust
    }

    var body: some View {
        VStack {
            TextField("Amount", text: $amount)
                .padding()
                .background(Color.gray.opacity(0.2))
                .cornerRadius(8)
                .padding()

            Button(action: {
                rust.dispatch(event: .pushRoute(route: .mintConfirm))
            }) {
                Text("Mint")
                    .frame(width: 100, height: 50)
                    .background(Color.blue)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }
            .padding()
        }
        .navigationTitle("Enter Amount")
    }
}

struct MintConfirmView: View {
    @Bindable var rust: ViewModel

    init(rust: ViewModel) {
        self.rust = rust
    }

    var body: some View {
        VStack {
            Text("Confirm Mint")
                .font(.title)
                .padding()

            Text("12")
                .font(.largeTitle)
                .padding()

            Button(action: {
                rust.dispatch(event: .pushRoute(route: .success))
            }) {
                Text("Confirm")
                    .frame(width: 120, height: 50)
                    .background(Color.green)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }
            .padding()
        }
        .navigationTitle("Confirm")
    }
}

struct MeltView: View {
    @Bindable var rust: ViewModel

    init(rust: ViewModel) {
        self.rust = rust
    }

    var body: some View {
        VStack {
            Text("Scan or paste Lightning invoice")
                .font(.title2)
                .padding()

            Rectangle()
                .fill(Color.gray.opacity(0.2))
                .frame(height: 200)
                .overlay(
                    Image(systemName: "qrcode.viewfinder")
                        .resizable()
                        .aspectRatio(contentMode: .fit)
                        .frame(width: 80, height: 80)
                )

            Button(action: {
                rust.dispatch(event: .pushRoute(route: .meltConfirm))
            }) {
                Text("Scan")
                    .frame(width: 120, height: 50)
                    .background(Color.blue)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }
            .padding()
        }
        .navigationTitle("Melt")
    }
}

struct MeltConfirmView: View {
    @Bindable var rust: ViewModel

    init(rust: ViewModel) {
        self.rust = rust
    }

    var body: some View {
        VStack {
            Text("Send 21,000 sats to foo@bar.com?")
                .font(.title2)
                .multilineTextAlignment(.center)
                .padding()

            HStack(spacing: 30) {
                Button(action: {
                    rust.dispatch(event: .pushRoute(route: .success))
                }) {
                    Text("Yes")
                        .frame(width: 80, height: 50)
                        .background(Color.green)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                }

                Button(action: {
                    rust.dispatch(event: .popRoute)
                }) {
                    Text("No")
                        .frame(width: 80, height: 50)
                        .background(Color.red)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                }
            }
            .padding()
        }
        .navigationTitle("Confirm")
    }
}

struct SuccessView: View {
    @Bindable var rust: ViewModel
    var message: String
    var onDismiss: () -> Void

    init(rust: ViewModel, message: String, onDismiss: @escaping () -> Void) {
        self.rust = rust
        self.message = message
        self.onDismiss = onDismiss
    }

    var body: some View {
        VStack {
            Spacer()

            Image(systemName: "party.popper.fill")
                .resizable()
                .aspectRatio(contentMode: .fit)
                .frame(width: 100, height: 100)
                .foregroundColor(.green)

            Text("Success!")
                .font(.largeTitle)
                .bold()
                .padding()

            Text(message)
                .font(.title2)
                .multilineTextAlignment(.center)
                .padding()

            Spacer()

            Button(action: onDismiss) {
                Text("Done")
                    .frame(width: 120, height: 50)
                    .background(Color.green)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }
            .padding(.bottom, 40)
        }
        .background(Color.white)
        .edgesIgnoringSafeArea(.all)
    }
}

struct ErrorView: View {
    @Bindable var rust: ViewModel
    var error: String
    var onRetry: () -> Void
    var onQuit: () -> Void

    init(
        rust: ViewModel, error: String, onRetry: @escaping () -> Void, onQuit: @escaping () -> Void
    ) {
        self.rust = rust
        self.error = error
        self.onRetry = onRetry
        self.onQuit = onQuit
    }

    var body: some View {
        VStack {
            Spacer()

            Image(systemName: "exclamationmark.triangle.fill")
                .resizable()
                .aspectRatio(contentMode: .fit)
                .frame(width: 100, height: 100)
                .foregroundColor(.red)

            Text("Error")
                .font(.largeTitle)
                .bold()
                .padding()

            Text(error)
                .font(.title2)
                .multilineTextAlignment(.center)
                .padding()

            Spacer()

            HStack(spacing: 30) {
                Button(action: onRetry) {
                    Text("Retry")
                        .frame(width: 120, height: 50)
                        .background(Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                }

                Button(action: onQuit) {
                    Text("Quit")
                        .frame(width: 120, height: 50)
                        .background(Color.red)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                }
            }
            .padding(.bottom, 40)
        }
        .background(Color.white)
        .edgesIgnoringSafeArea(.all)
    }
}
