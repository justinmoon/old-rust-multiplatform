import Counter
import SwiftUI

struct AppNavigation: View {
    @State var rust: ViewModel
    @State private var showErrorScreen = false
    @State private var showSuccessScreen = false
    @State private var successMessage = "Transaction completed successfully!"
    @State private var errorMessage = "Transaction failed. Please try again."

    // Public initializer
    init(rust: ViewModel) {
        self._rust = State(initialValue: rust)
        print("AppNavigation initialized with ViewModel ID: \(ObjectIdentifier(rust))")
    }

    var body: some View {
        // Create a read-only binding for navigation path derived from rust.router.route
        NavigationStack(
            path: Binding(
                get: {
                    // Convert router.route to navigation path
                    if rust.router.route != .home {
                        return [rust.router.route]
                    } else {
                        return []
                    }
                },
                set: { _ in
                    // Intentionally empty - we only update through rust.dispatch
                }
            )
        ) {
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
        .onChange(of: rust.router.route) { _, newRoute in
            // Handle special routes
            if newRoute == .success {
                // Show success screen
                showSuccessScreen = true
            } else if newRoute == .error {
                // Show error screen
                showErrorScreen = true
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
                    rust.dispatch(event: .resetNavigationStack)
                    showErrorScreen = false
                }
            )
        }
        .fullScreenCover(isPresented: $showSuccessScreen) {
            SuccessView(rust: rust, message: successMessage) {
                // Reset to home on dismiss
                rust.dispatch(event: .resetNavigationStack)
                showSuccessScreen = false
            }
        }
    }
}

// MARK: - View Implementations

struct HomeView: View {
    @State var rust: ViewModel

    init(rust: ViewModel) {
        self._rust = State(initialValue: rust)
        print("HomeView initialized with ViewModel ID: \(ObjectIdentifier(rust))")
    }

    var body: some View {
        VStack {
            Text("21,000 sats")
                .font(.largeTitle)
                .padding()

            HStack(spacing: 20) {
                Button(action: {
                    rust.dispatch(event: .pushRoute(route: .mint))
                }) {
                    Text("Mint")
                        .frame(width: 100, height: 50)
                        .background(Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                }

                Button(action: {
                    rust.dispatch(event: .pushRoute(route: .melt))
                }) {
                    Text("Melt")
                        .frame(width: 100, height: 50)
                        .background(Color.red)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                }
            }

            // Add animation for jiggling buttons when transaction exists
            // as shown in your mockup
            .padding()
        }
        .navigationTitle("Home")
    }
}

struct TransactionHistoryView: View {
    @State var rust: ViewModel

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
    @State var rust: ViewModel

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
    @State var rust: ViewModel
    @State private var amount: String = "12"

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
    @State var rust: ViewModel

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
    @State var rust: ViewModel

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
    @State var rust: ViewModel

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
    @State var rust: ViewModel
    var message: String
    var onDismiss: () -> Void

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
    @State var rust: ViewModel
    var error: String
    var onRetry: () -> Void
    var onQuit: () -> Void

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
