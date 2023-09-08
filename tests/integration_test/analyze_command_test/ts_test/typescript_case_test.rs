use crate::integration_test::assert_analyzed_source_code;
use indoc::indoc;

#[test]
fn test_typescript_functions() {
    let source_code = indoc! {r#"
    function parseBindingIdentifier(privateIdentifierDiagnosticMessage?: DiagnosticMessage) {
        return createIdentifier(isBindingIdentifier(), /*diagnosticMessage*/ undefined, privateIdentifierDiagnosticMessage);
    }

    export function parseIdentifier(diagnosticMessage?: DiagnosticMessage, privateIdentifierDiagnosticMessage?: DiagnosticMessage): Identifier {
        return createIdentifier(isIdentifier(), diagnosticMessage, privateIdentifierDiagnosticMessage);
    }

    export function parseIdentifierName(diagnosticMessage?: DiagnosticMessage): Identifier {
        return createIdentifier(tokenIsIdentifierOrKeyword(token()), diagnosticMessage);
    }"#};

    let expected = indoc! {r#"
    // [TODO] parseBindingIdentifier
    function parseBindingIdentifier(privateIdentifierDiagnosticMessage?: DiagnosticMessage) {
        return createIdentifier(isBindingIdentifier(), /*diagnosticMessage*/ undefined, privateIdentifierDiagnosticMessage);
    }

    // [TODO] parseIdentifier
    export function parseIdentifier(diagnosticMessage?: DiagnosticMessage, privateIdentifierDiagnosticMessage?: DiagnosticMessage): Identifier {
        return createIdentifier(isIdentifier(), diagnosticMessage, privateIdentifierDiagnosticMessage);
    }

    // [TODO] parseIdentifierName
    export function parseIdentifierName(diagnosticMessage?: DiagnosticMessage): Identifier {
        return createIdentifier(tokenIsIdentifierOrKeyword(token()), diagnosticMessage);
    }"#};

    assert_analyzed_source_code(source_code, expected, "typescript")
}

#[test]
fn test_multiple_interfaces() {
    let source_code = indoc! {r#"
    export interface EmitOutput {
        outputFiles: OutputFile[];
        emitSkipped: boolean;
        /** @internal */ diagnostics: readonly Diagnostic[];
    }
    
    interface OutputFile {
        name: string;
        writeByteOrderMark: boolean;
        text: string;
        /** @internal */ data?: WriteFileCallbackData;
    }"#};

    let expected = indoc! {r#"
    // [TODO] EmitOutput
    export interface EmitOutput {
        outputFiles: OutputFile[];
        emitSkipped: boolean;
        /** @internal */ diagnostics: readonly Diagnostic[];
    }
    
    // [TODO] OutputFile
    interface OutputFile {
        name: string;
        writeByteOrderMark: boolean;
        text: string;
        /** @internal */ data?: WriteFileCallbackData;
    }"#};

    assert_analyzed_source_code(source_code, expected, "typescript")
}

#[test]
#[ignore = "Not supported yet"]
fn test_arrow_function() {
    let source_code = indoc! {r#"
    export const foo = <T extends {}>(myObject: T) => myObject.toString();"#};
    
    let expected = indoc! {r#"
    // [TODO] foo
    export const foo = <T extends {}>(myObject: T) => myObject.toString();"#};

    assert_analyzed_source_code(source_code, expected, "typescript")
}

#[test]
// TODO: support export class
fn test_class_declaration_without_export_symbol() {
    let source_code = indoc! {r#"
    class Person {
        private name: string;

        public constructor(name: string) {
            this.name = name;
        }

        public getName(): string {
            return this.name;
        }
    }"#};

    let expected = indoc! {r#"
    // [TODO] Person
    class Person {
        private name: string;

        // [TODO] Person > constructor
        public constructor(name: string) {
            this.name = name;
        }

        // [TODO] Person > getName
        public getName(): string {
            return this.name;
        }
    }"#};

    assert_analyzed_source_code(source_code, expected, "typescript")
}

#[test]
fn test_class_extend_with_implementation() {
    let source_code = indoc! {r#"
    export interface Pingable {
        ping(): void;
    }

    class Sonar implements Pingable {
        ping() {
            console.log("ping!");
        }
    }"#};

    let expected = indoc! {r#"
    // [TODO] Pingable
    export interface Pingable {
        ping(): void;
    }

    // [TODO] Sonar
    class Sonar implements Pingable {
        // [TODO] Sonar > ping
        ping() {
            console.log("ping!");
        }
    }"#};

    assert_analyzed_source_code(source_code, expected, "typescript")
}

#[test]
#[ignore = "Not supported yet"]
fn test_namespace() {
    let source_code = indoc! {r#"
    namespace Animal {
        function animalsHaveMuscles() {
            return foo;
        }
    }"#};

    let expected = indoc! {r#"
    // [TODO] Animal
    namespace Animal {
        // [TODO] Animal > animalsHaveMuscles
        function animalsHaveMuscles() {
            return foo;
        }
    }"#};

    assert_analyzed_source_code(source_code, expected, "typescript")
}