import * as $protobuf from "protobufjs";
/** Properties of a CouncilCandidacyNoteMetadata. */
export interface ICouncilCandidacyNoteMetadata {

    /** CouncilCandidacyNoteMetadata header */
    header?: (string|null);

    /** CouncilCandidacyNoteMetadata bulletPoints */
    bulletPoints?: (string[]|null);

    /** CouncilCandidacyNoteMetadata bannerImageUri */
    bannerImageUri?: (string|null);

    /** CouncilCandidacyNoteMetadata description */
    description?: (string|null);
}

/** Represents a CouncilCandidacyNoteMetadata. */
export class CouncilCandidacyNoteMetadata implements ICouncilCandidacyNoteMetadata {

    /**
     * Constructs a new CouncilCandidacyNoteMetadata.
     * @param [properties] Properties to set
     */
    constructor(properties?: ICouncilCandidacyNoteMetadata);

    /** CouncilCandidacyNoteMetadata header. */
    public header: string;

    /** CouncilCandidacyNoteMetadata bulletPoints. */
    public bulletPoints: string[];

    /** CouncilCandidacyNoteMetadata bannerImageUri. */
    public bannerImageUri: string;

    /** CouncilCandidacyNoteMetadata description. */
    public description: string;

    /**
     * Creates a new CouncilCandidacyNoteMetadata instance using the specified properties.
     * @param [properties] Properties to set
     * @returns CouncilCandidacyNoteMetadata instance
     */
    public static create(properties?: ICouncilCandidacyNoteMetadata): CouncilCandidacyNoteMetadata;

    /**
     * Encodes the specified CouncilCandidacyNoteMetadata message. Does not implicitly {@link CouncilCandidacyNoteMetadata.verify|verify} messages.
     * @param message CouncilCandidacyNoteMetadata message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: ICouncilCandidacyNoteMetadata, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified CouncilCandidacyNoteMetadata message, length delimited. Does not implicitly {@link CouncilCandidacyNoteMetadata.verify|verify} messages.
     * @param message CouncilCandidacyNoteMetadata message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: ICouncilCandidacyNoteMetadata, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a CouncilCandidacyNoteMetadata message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns CouncilCandidacyNoteMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): CouncilCandidacyNoteMetadata;

    /**
     * Decodes a CouncilCandidacyNoteMetadata message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns CouncilCandidacyNoteMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): CouncilCandidacyNoteMetadata;

    /**
     * Verifies a CouncilCandidacyNoteMetadata message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a CouncilCandidacyNoteMetadata message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns CouncilCandidacyNoteMetadata
     */
    public static fromObject(object: { [k: string]: any }): CouncilCandidacyNoteMetadata;

    /**
     * Creates a plain object from a CouncilCandidacyNoteMetadata message. Also converts values to other types if specified.
     * @param message CouncilCandidacyNoteMetadata
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: CouncilCandidacyNoteMetadata, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this CouncilCandidacyNoteMetadata to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a ForumPostReaction. */
export interface IForumPostReaction {
}

/** Represents a ForumPostReaction. */
export class ForumPostReaction implements IForumPostReaction {

    /**
     * Constructs a new ForumPostReaction.
     * @param [properties] Properties to set
     */
    constructor(properties?: IForumPostReaction);

    /**
     * Creates a new ForumPostReaction instance using the specified properties.
     * @param [properties] Properties to set
     * @returns ForumPostReaction instance
     */
    public static create(properties?: IForumPostReaction): ForumPostReaction;

    /**
     * Encodes the specified ForumPostReaction message. Does not implicitly {@link ForumPostReaction.verify|verify} messages.
     * @param message ForumPostReaction message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: IForumPostReaction, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified ForumPostReaction message, length delimited. Does not implicitly {@link ForumPostReaction.verify|verify} messages.
     * @param message ForumPostReaction message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: IForumPostReaction, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a ForumPostReaction message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns ForumPostReaction
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): ForumPostReaction;

    /**
     * Decodes a ForumPostReaction message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns ForumPostReaction
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): ForumPostReaction;

    /**
     * Verifies a ForumPostReaction message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a ForumPostReaction message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns ForumPostReaction
     */
    public static fromObject(object: { [k: string]: any }): ForumPostReaction;

    /**
     * Creates a plain object from a ForumPostReaction message. Also converts values to other types if specified.
     * @param message ForumPostReaction
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: ForumPostReaction, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this ForumPostReaction to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

export namespace ForumPostReaction {

    /** Reaction enum. */
    enum Reaction {
        CANCEL = 0,
        LIKE = 1
    }
}

/** Properties of a ForumPostMetadata. */
export interface IForumPostMetadata {

    /** ForumPostMetadata text */
    text?: (string|null);

    /** ForumPostMetadata repliesTo */
    repliesTo?: (number|null);
}

/** Represents a ForumPostMetadata. */
export class ForumPostMetadata implements IForumPostMetadata {

    /**
     * Constructs a new ForumPostMetadata.
     * @param [properties] Properties to set
     */
    constructor(properties?: IForumPostMetadata);

    /** ForumPostMetadata text. */
    public text: string;

    /** ForumPostMetadata repliesTo. */
    public repliesTo: number;

    /**
     * Creates a new ForumPostMetadata instance using the specified properties.
     * @param [properties] Properties to set
     * @returns ForumPostMetadata instance
     */
    public static create(properties?: IForumPostMetadata): ForumPostMetadata;

    /**
     * Encodes the specified ForumPostMetadata message. Does not implicitly {@link ForumPostMetadata.verify|verify} messages.
     * @param message ForumPostMetadata message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: IForumPostMetadata, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified ForumPostMetadata message, length delimited. Does not implicitly {@link ForumPostMetadata.verify|verify} messages.
     * @param message ForumPostMetadata message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: IForumPostMetadata, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a ForumPostMetadata message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns ForumPostMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): ForumPostMetadata;

    /**
     * Decodes a ForumPostMetadata message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns ForumPostMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): ForumPostMetadata;

    /**
     * Verifies a ForumPostMetadata message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a ForumPostMetadata message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns ForumPostMetadata
     */
    public static fromObject(object: { [k: string]: any }): ForumPostMetadata;

    /**
     * Creates a plain object from a ForumPostMetadata message. Also converts values to other types if specified.
     * @param message ForumPostMetadata
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: ForumPostMetadata, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this ForumPostMetadata to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a MembershipMetadata. */
export interface IMembershipMetadata {

    /** MembershipMetadata name */
    name?: (string|null);

    /** MembershipMetadata avatar */
    avatar?: (number|null);

    /** MembershipMetadata about */
    about?: (string|null);
}

/** Represents a MembershipMetadata. */
export class MembershipMetadata implements IMembershipMetadata {

    /**
     * Constructs a new MembershipMetadata.
     * @param [properties] Properties to set
     */
    constructor(properties?: IMembershipMetadata);

    /** MembershipMetadata name. */
    public name: string;

    /** MembershipMetadata avatar. */
    public avatar: number;

    /** MembershipMetadata about. */
    public about: string;

    /**
     * Creates a new MembershipMetadata instance using the specified properties.
     * @param [properties] Properties to set
     * @returns MembershipMetadata instance
     */
    public static create(properties?: IMembershipMetadata): MembershipMetadata;

    /**
     * Encodes the specified MembershipMetadata message. Does not implicitly {@link MembershipMetadata.verify|verify} messages.
     * @param message MembershipMetadata message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: IMembershipMetadata, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified MembershipMetadata message, length delimited. Does not implicitly {@link MembershipMetadata.verify|verify} messages.
     * @param message MembershipMetadata message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: IMembershipMetadata, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a MembershipMetadata message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns MembershipMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): MembershipMetadata;

    /**
     * Decodes a MembershipMetadata message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns MembershipMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): MembershipMetadata;

    /**
     * Verifies a MembershipMetadata message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a MembershipMetadata message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns MembershipMetadata
     */
    public static fromObject(object: { [k: string]: any }): MembershipMetadata;

    /**
     * Creates a plain object from a MembershipMetadata message. Also converts values to other types if specified.
     * @param message MembershipMetadata
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: MembershipMetadata, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this MembershipMetadata to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of an OpeningMetadata. */
export interface IOpeningMetadata {

    /** OpeningMetadata shortDescription */
    shortDescription?: (string|null);

    /** OpeningMetadata description */
    description?: (string|null);

    /** OpeningMetadata hiringLimit */
    hiringLimit?: (number|null);

    /** OpeningMetadata expectedEndingTimestamp */
    expectedEndingTimestamp?: (number|null);

    /** OpeningMetadata applicationDetails */
    applicationDetails?: (string|null);

    /** OpeningMetadata applicationFormQuestions */
    applicationFormQuestions?: (OpeningMetadata.IApplicationFormQuestion[]|null);
}

/** Represents an OpeningMetadata. */
export class OpeningMetadata implements IOpeningMetadata {

    /**
     * Constructs a new OpeningMetadata.
     * @param [properties] Properties to set
     */
    constructor(properties?: IOpeningMetadata);

    /** OpeningMetadata shortDescription. */
    public shortDescription: string;

    /** OpeningMetadata description. */
    public description: string;

    /** OpeningMetadata hiringLimit. */
    public hiringLimit: number;

    /** OpeningMetadata expectedEndingTimestamp. */
    public expectedEndingTimestamp: number;

    /** OpeningMetadata applicationDetails. */
    public applicationDetails: string;

    /** OpeningMetadata applicationFormQuestions. */
    public applicationFormQuestions: OpeningMetadata.IApplicationFormQuestion[];

    /**
     * Creates a new OpeningMetadata instance using the specified properties.
     * @param [properties] Properties to set
     * @returns OpeningMetadata instance
     */
    public static create(properties?: IOpeningMetadata): OpeningMetadata;

    /**
     * Encodes the specified OpeningMetadata message. Does not implicitly {@link OpeningMetadata.verify|verify} messages.
     * @param message OpeningMetadata message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: IOpeningMetadata, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified OpeningMetadata message, length delimited. Does not implicitly {@link OpeningMetadata.verify|verify} messages.
     * @param message OpeningMetadata message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: IOpeningMetadata, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes an OpeningMetadata message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns OpeningMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): OpeningMetadata;

    /**
     * Decodes an OpeningMetadata message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns OpeningMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): OpeningMetadata;

    /**
     * Verifies an OpeningMetadata message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates an OpeningMetadata message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns OpeningMetadata
     */
    public static fromObject(object: { [k: string]: any }): OpeningMetadata;

    /**
     * Creates a plain object from an OpeningMetadata message. Also converts values to other types if specified.
     * @param message OpeningMetadata
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: OpeningMetadata, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this OpeningMetadata to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

export namespace OpeningMetadata {

    /** Properties of an ApplicationFormQuestion. */
    interface IApplicationFormQuestion {

        /** ApplicationFormQuestion question */
        question?: (string|null);

        /** ApplicationFormQuestion type */
        type?: (OpeningMetadata.ApplicationFormQuestion.InputType|null);
    }

    /** Represents an ApplicationFormQuestion. */
    class ApplicationFormQuestion implements IApplicationFormQuestion {

        /**
         * Constructs a new ApplicationFormQuestion.
         * @param [properties] Properties to set
         */
        constructor(properties?: OpeningMetadata.IApplicationFormQuestion);

        /** ApplicationFormQuestion question. */
        public question: string;

        /** ApplicationFormQuestion type. */
        public type: OpeningMetadata.ApplicationFormQuestion.InputType;

        /**
         * Creates a new ApplicationFormQuestion instance using the specified properties.
         * @param [properties] Properties to set
         * @returns ApplicationFormQuestion instance
         */
        public static create(properties?: OpeningMetadata.IApplicationFormQuestion): OpeningMetadata.ApplicationFormQuestion;

        /**
         * Encodes the specified ApplicationFormQuestion message. Does not implicitly {@link OpeningMetadata.ApplicationFormQuestion.verify|verify} messages.
         * @param message ApplicationFormQuestion message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: OpeningMetadata.IApplicationFormQuestion, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified ApplicationFormQuestion message, length delimited. Does not implicitly {@link OpeningMetadata.ApplicationFormQuestion.verify|verify} messages.
         * @param message ApplicationFormQuestion message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: OpeningMetadata.IApplicationFormQuestion, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes an ApplicationFormQuestion message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns ApplicationFormQuestion
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): OpeningMetadata.ApplicationFormQuestion;

        /**
         * Decodes an ApplicationFormQuestion message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns ApplicationFormQuestion
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): OpeningMetadata.ApplicationFormQuestion;

        /**
         * Verifies an ApplicationFormQuestion message.
         * @param message Plain object to verify
         * @returns `null` if valid, otherwise the reason why it is not
         */
        public static verify(message: { [k: string]: any }): (string|null);

        /**
         * Creates an ApplicationFormQuestion message from a plain object. Also converts values to their respective internal types.
         * @param object Plain object
         * @returns ApplicationFormQuestion
         */
        public static fromObject(object: { [k: string]: any }): OpeningMetadata.ApplicationFormQuestion;

        /**
         * Creates a plain object from an ApplicationFormQuestion message. Also converts values to other types if specified.
         * @param message ApplicationFormQuestion
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: OpeningMetadata.ApplicationFormQuestion, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this ApplicationFormQuestion to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    namespace ApplicationFormQuestion {

        /** InputType enum. */
        enum InputType {
            TEXTAREA = 0,
            TEXT = 1
        }
    }
}

/** Properties of an UpcomingOpeningMetadata. */
export interface IUpcomingOpeningMetadata {

    /** UpcomingOpeningMetadata expectedStart */
    expectedStart?: (number|null);

    /** UpcomingOpeningMetadata rewardPerBlock */
    rewardPerBlock?: (Long|null);

    /** UpcomingOpeningMetadata minApplicationStake */
    minApplicationStake?: (Long|null);

    /** UpcomingOpeningMetadata metadata */
    metadata?: (IOpeningMetadata|null);
}

/** Represents an UpcomingOpeningMetadata. */
export class UpcomingOpeningMetadata implements IUpcomingOpeningMetadata {

    /**
     * Constructs a new UpcomingOpeningMetadata.
     * @param [properties] Properties to set
     */
    constructor(properties?: IUpcomingOpeningMetadata);

    /** UpcomingOpeningMetadata expectedStart. */
    public expectedStart: number;

    /** UpcomingOpeningMetadata rewardPerBlock. */
    public rewardPerBlock: Long;

    /** UpcomingOpeningMetadata minApplicationStake. */
    public minApplicationStake: Long;

    /** UpcomingOpeningMetadata metadata. */
    public metadata?: (IOpeningMetadata|null);

    /**
     * Creates a new UpcomingOpeningMetadata instance using the specified properties.
     * @param [properties] Properties to set
     * @returns UpcomingOpeningMetadata instance
     */
    public static create(properties?: IUpcomingOpeningMetadata): UpcomingOpeningMetadata;

    /**
     * Encodes the specified UpcomingOpeningMetadata message. Does not implicitly {@link UpcomingOpeningMetadata.verify|verify} messages.
     * @param message UpcomingOpeningMetadata message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: IUpcomingOpeningMetadata, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified UpcomingOpeningMetadata message, length delimited. Does not implicitly {@link UpcomingOpeningMetadata.verify|verify} messages.
     * @param message UpcomingOpeningMetadata message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: IUpcomingOpeningMetadata, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes an UpcomingOpeningMetadata message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns UpcomingOpeningMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): UpcomingOpeningMetadata;

    /**
     * Decodes an UpcomingOpeningMetadata message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns UpcomingOpeningMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): UpcomingOpeningMetadata;

    /**
     * Verifies an UpcomingOpeningMetadata message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates an UpcomingOpeningMetadata message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns UpcomingOpeningMetadata
     */
    public static fromObject(object: { [k: string]: any }): UpcomingOpeningMetadata;

    /**
     * Creates a plain object from an UpcomingOpeningMetadata message. Also converts values to other types if specified.
     * @param message UpcomingOpeningMetadata
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: UpcomingOpeningMetadata, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this UpcomingOpeningMetadata to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of an ApplicationMetadata. */
export interface IApplicationMetadata {

    /** ApplicationMetadata answers */
    answers?: (string[]|null);
}

/** Represents an ApplicationMetadata. */
export class ApplicationMetadata implements IApplicationMetadata {

    /**
     * Constructs a new ApplicationMetadata.
     * @param [properties] Properties to set
     */
    constructor(properties?: IApplicationMetadata);

    /** ApplicationMetadata answers. */
    public answers: string[];

    /**
     * Creates a new ApplicationMetadata instance using the specified properties.
     * @param [properties] Properties to set
     * @returns ApplicationMetadata instance
     */
    public static create(properties?: IApplicationMetadata): ApplicationMetadata;

    /**
     * Encodes the specified ApplicationMetadata message. Does not implicitly {@link ApplicationMetadata.verify|verify} messages.
     * @param message ApplicationMetadata message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: IApplicationMetadata, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified ApplicationMetadata message, length delimited. Does not implicitly {@link ApplicationMetadata.verify|verify} messages.
     * @param message ApplicationMetadata message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: IApplicationMetadata, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes an ApplicationMetadata message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns ApplicationMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): ApplicationMetadata;

    /**
     * Decodes an ApplicationMetadata message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns ApplicationMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): ApplicationMetadata;

    /**
     * Verifies an ApplicationMetadata message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates an ApplicationMetadata message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns ApplicationMetadata
     */
    public static fromObject(object: { [k: string]: any }): ApplicationMetadata;

    /**
     * Creates a plain object from an ApplicationMetadata message. Also converts values to other types if specified.
     * @param message ApplicationMetadata
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: ApplicationMetadata, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this ApplicationMetadata to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a WorkingGroupMetadata. */
export interface IWorkingGroupMetadata {

    /** WorkingGroupMetadata description */
    description?: (string|null);

    /** WorkingGroupMetadata about */
    about?: (string|null);

    /** WorkingGroupMetadata status */
    status?: (string|null);

    /** WorkingGroupMetadata statusMessage */
    statusMessage?: (string|null);
}

/** Represents a WorkingGroupMetadata. */
export class WorkingGroupMetadata implements IWorkingGroupMetadata {

    /**
     * Constructs a new WorkingGroupMetadata.
     * @param [properties] Properties to set
     */
    constructor(properties?: IWorkingGroupMetadata);

    /** WorkingGroupMetadata description. */
    public description: string;

    /** WorkingGroupMetadata about. */
    public about: string;

    /** WorkingGroupMetadata status. */
    public status: string;

    /** WorkingGroupMetadata statusMessage. */
    public statusMessage: string;

    /**
     * Creates a new WorkingGroupMetadata instance using the specified properties.
     * @param [properties] Properties to set
     * @returns WorkingGroupMetadata instance
     */
    public static create(properties?: IWorkingGroupMetadata): WorkingGroupMetadata;

    /**
     * Encodes the specified WorkingGroupMetadata message. Does not implicitly {@link WorkingGroupMetadata.verify|verify} messages.
     * @param message WorkingGroupMetadata message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: IWorkingGroupMetadata, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified WorkingGroupMetadata message, length delimited. Does not implicitly {@link WorkingGroupMetadata.verify|verify} messages.
     * @param message WorkingGroupMetadata message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: IWorkingGroupMetadata, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a WorkingGroupMetadata message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns WorkingGroupMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): WorkingGroupMetadata;

    /**
     * Decodes a WorkingGroupMetadata message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns WorkingGroupMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): WorkingGroupMetadata;

    /**
     * Verifies a WorkingGroupMetadata message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a WorkingGroupMetadata message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns WorkingGroupMetadata
     */
    public static fromObject(object: { [k: string]: any }): WorkingGroupMetadata;

    /**
     * Creates a plain object from a WorkingGroupMetadata message. Also converts values to other types if specified.
     * @param message WorkingGroupMetadata
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: WorkingGroupMetadata, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this WorkingGroupMetadata to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a SetGroupMetadata. */
export interface ISetGroupMetadata {

    /** SetGroupMetadata newMetadata */
    newMetadata?: (IWorkingGroupMetadata|null);
}

/** Represents a SetGroupMetadata. */
export class SetGroupMetadata implements ISetGroupMetadata {

    /**
     * Constructs a new SetGroupMetadata.
     * @param [properties] Properties to set
     */
    constructor(properties?: ISetGroupMetadata);

    /** SetGroupMetadata newMetadata. */
    public newMetadata?: (IWorkingGroupMetadata|null);

    /**
     * Creates a new SetGroupMetadata instance using the specified properties.
     * @param [properties] Properties to set
     * @returns SetGroupMetadata instance
     */
    public static create(properties?: ISetGroupMetadata): SetGroupMetadata;

    /**
     * Encodes the specified SetGroupMetadata message. Does not implicitly {@link SetGroupMetadata.verify|verify} messages.
     * @param message SetGroupMetadata message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: ISetGroupMetadata, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified SetGroupMetadata message, length delimited. Does not implicitly {@link SetGroupMetadata.verify|verify} messages.
     * @param message SetGroupMetadata message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: ISetGroupMetadata, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a SetGroupMetadata message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns SetGroupMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): SetGroupMetadata;

    /**
     * Decodes a SetGroupMetadata message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns SetGroupMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): SetGroupMetadata;

    /**
     * Verifies a SetGroupMetadata message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a SetGroupMetadata message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns SetGroupMetadata
     */
    public static fromObject(object: { [k: string]: any }): SetGroupMetadata;

    /**
     * Creates a plain object from a SetGroupMetadata message. Also converts values to other types if specified.
     * @param message SetGroupMetadata
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: SetGroupMetadata, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this SetGroupMetadata to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of an AddUpcomingOpening. */
export interface IAddUpcomingOpening {

    /** AddUpcomingOpening metadata */
    metadata?: (IUpcomingOpeningMetadata|null);
}

/** Represents an AddUpcomingOpening. */
export class AddUpcomingOpening implements IAddUpcomingOpening {

    /**
     * Constructs a new AddUpcomingOpening.
     * @param [properties] Properties to set
     */
    constructor(properties?: IAddUpcomingOpening);

    /** AddUpcomingOpening metadata. */
    public metadata?: (IUpcomingOpeningMetadata|null);

    /**
     * Creates a new AddUpcomingOpening instance using the specified properties.
     * @param [properties] Properties to set
     * @returns AddUpcomingOpening instance
     */
    public static create(properties?: IAddUpcomingOpening): AddUpcomingOpening;

    /**
     * Encodes the specified AddUpcomingOpening message. Does not implicitly {@link AddUpcomingOpening.verify|verify} messages.
     * @param message AddUpcomingOpening message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: IAddUpcomingOpening, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified AddUpcomingOpening message, length delimited. Does not implicitly {@link AddUpcomingOpening.verify|verify} messages.
     * @param message AddUpcomingOpening message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: IAddUpcomingOpening, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes an AddUpcomingOpening message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns AddUpcomingOpening
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): AddUpcomingOpening;

    /**
     * Decodes an AddUpcomingOpening message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns AddUpcomingOpening
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): AddUpcomingOpening;

    /**
     * Verifies an AddUpcomingOpening message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates an AddUpcomingOpening message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns AddUpcomingOpening
     */
    public static fromObject(object: { [k: string]: any }): AddUpcomingOpening;

    /**
     * Creates a plain object from an AddUpcomingOpening message. Also converts values to other types if specified.
     * @param message AddUpcomingOpening
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: AddUpcomingOpening, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this AddUpcomingOpening to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a RemoveUpcomingOpening. */
export interface IRemoveUpcomingOpening {

    /** RemoveUpcomingOpening id */
    id?: (string|null);
}

/** Represents a RemoveUpcomingOpening. */
export class RemoveUpcomingOpening implements IRemoveUpcomingOpening {

    /**
     * Constructs a new RemoveUpcomingOpening.
     * @param [properties] Properties to set
     */
    constructor(properties?: IRemoveUpcomingOpening);

    /** RemoveUpcomingOpening id. */
    public id: string;

    /**
     * Creates a new RemoveUpcomingOpening instance using the specified properties.
     * @param [properties] Properties to set
     * @returns RemoveUpcomingOpening instance
     */
    public static create(properties?: IRemoveUpcomingOpening): RemoveUpcomingOpening;

    /**
     * Encodes the specified RemoveUpcomingOpening message. Does not implicitly {@link RemoveUpcomingOpening.verify|verify} messages.
     * @param message RemoveUpcomingOpening message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: IRemoveUpcomingOpening, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified RemoveUpcomingOpening message, length delimited. Does not implicitly {@link RemoveUpcomingOpening.verify|verify} messages.
     * @param message RemoveUpcomingOpening message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: IRemoveUpcomingOpening, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a RemoveUpcomingOpening message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns RemoveUpcomingOpening
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): RemoveUpcomingOpening;

    /**
     * Decodes a RemoveUpcomingOpening message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns RemoveUpcomingOpening
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): RemoveUpcomingOpening;

    /**
     * Verifies a RemoveUpcomingOpening message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a RemoveUpcomingOpening message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns RemoveUpcomingOpening
     */
    public static fromObject(object: { [k: string]: any }): RemoveUpcomingOpening;

    /**
     * Creates a plain object from a RemoveUpcomingOpening message. Also converts values to other types if specified.
     * @param message RemoveUpcomingOpening
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: RemoveUpcomingOpening, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this RemoveUpcomingOpening to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a WorkingGroupMetadataAction. */
export interface IWorkingGroupMetadataAction {

    /** WorkingGroupMetadataAction setGroupMetadata */
    setGroupMetadata?: (ISetGroupMetadata|null);

    /** WorkingGroupMetadataAction addUpcomingOpening */
    addUpcomingOpening?: (IAddUpcomingOpening|null);

    /** WorkingGroupMetadataAction removeUpcomingOpening */
    removeUpcomingOpening?: (IRemoveUpcomingOpening|null);
}

/** Represents a WorkingGroupMetadataAction. */
export class WorkingGroupMetadataAction implements IWorkingGroupMetadataAction {

    /**
     * Constructs a new WorkingGroupMetadataAction.
     * @param [properties] Properties to set
     */
    constructor(properties?: IWorkingGroupMetadataAction);

    /** WorkingGroupMetadataAction setGroupMetadata. */
    public setGroupMetadata?: (ISetGroupMetadata|null);

    /** WorkingGroupMetadataAction addUpcomingOpening. */
    public addUpcomingOpening?: (IAddUpcomingOpening|null);

    /** WorkingGroupMetadataAction removeUpcomingOpening. */
    public removeUpcomingOpening?: (IRemoveUpcomingOpening|null);

    /** WorkingGroupMetadataAction action. */
    public action?: ("setGroupMetadata"|"addUpcomingOpening"|"removeUpcomingOpening");

    /**
     * Creates a new WorkingGroupMetadataAction instance using the specified properties.
     * @param [properties] Properties to set
     * @returns WorkingGroupMetadataAction instance
     */
    public static create(properties?: IWorkingGroupMetadataAction): WorkingGroupMetadataAction;

    /**
     * Encodes the specified WorkingGroupMetadataAction message. Does not implicitly {@link WorkingGroupMetadataAction.verify|verify} messages.
     * @param message WorkingGroupMetadataAction message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: IWorkingGroupMetadataAction, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified WorkingGroupMetadataAction message, length delimited. Does not implicitly {@link WorkingGroupMetadataAction.verify|verify} messages.
     * @param message WorkingGroupMetadataAction message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: IWorkingGroupMetadataAction, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a WorkingGroupMetadataAction message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns WorkingGroupMetadataAction
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): WorkingGroupMetadataAction;

    /**
     * Decodes a WorkingGroupMetadataAction message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns WorkingGroupMetadataAction
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): WorkingGroupMetadataAction;

    /**
     * Verifies a WorkingGroupMetadataAction message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a WorkingGroupMetadataAction message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns WorkingGroupMetadataAction
     */
    public static fromObject(object: { [k: string]: any }): WorkingGroupMetadataAction;

    /**
     * Creates a plain object from a WorkingGroupMetadataAction message. Also converts values to other types if specified.
     * @param message WorkingGroupMetadataAction
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: WorkingGroupMetadataAction, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this WorkingGroupMetadataAction to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}
